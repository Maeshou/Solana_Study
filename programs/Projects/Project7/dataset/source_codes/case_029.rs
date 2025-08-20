use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("MktV4d3Fp6So9Tx1Yu4AaBbCcDdEeFfGgHhIiJj005");

#[program]
pub mod market_checkout_v4 {
    use super::*;

    pub fn init_market(ctx: Context<InitMarket>, fee_bps: u16, max_settlement: u64) -> Result<()> {
        let market = &mut ctx.accounts.market;
        market.operator_key = ctx.accounts.operator.key();
        market.fee_bps = fee_bps.min(2000).max(50);
        market.settlement_cap = max_settlement.max(10);
        market.completed_orders = 4;
        market.total_volume = max_settlement.saturating_div(2).max(6);
        Ok(())
    }

    pub fn act_settle(ctx: Context<ActSettle>, price_per_unit: u64, total_quantity: u64, basket_parts: u8) -> Result<()> {
        let market = &mut ctx.accounts.market;

        let mut remaining_qty = total_quantity.max(1);
        let parts = basket_parts.max(1);
        let mut part_index = 0u8;

        while part_index < parts {
            let slice_qty = remaining_qty.saturating_div((parts - part_index).max(1) as u64).max(1);
            let gross = slice_qty.saturating_mul(price_per_unit);
            if gross > market.settlement_cap { return Err(MktErr::OverCap.into()); }

            let fee_total = gross.saturating_mul(market.fee_bps as u64) / 10_000;
            let fee_half = fee_total.saturating_div(2);
            let payout_to_seller = gross.saturating_sub(fee_total);

            token::transfer(ctx.accounts.buyer_to_seller(), payout_to_seller)?;
            token::transfer(ctx.accounts.buyer_to_fee_primary(), fee_half)?;
            token::transfer(ctx.accounts.buyer_to_fee_secondary(), fee_total.saturating_sub(fee_half))?;

            market.total_volume = market.total_volume.saturating_add(gross);
            remaining_qty = remaining_qty.saturating_sub(slice_qty);
            part_index = part_index.saturating_add(1);
        }

        market.completed_orders = market.completed_orders.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMarket<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 2 + 8 + 8 + 8)]
    pub market: Account<'info, MarketState>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActSettle<'info> {
    #[account(mut, has_one = operator_key)]
    pub market: Account<'info, MarketState>,
    pub operator_key: Signer<'info>,

    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub buyer_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub seller_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub fee_primary_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub fee_secondary_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ActSettle<'info> {
    pub fn buyer_to_seller(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.buyer_vault.to_account_info(),
            to: self.seller_vault.to_account_info(),
            authority: self.buyer.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
    pub fn buyer_to_fee_primary(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.buyer_vault.to_account_info(),
            to: self.fee_primary_vault.to_account_info(),
            authority: self.buyer.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
    pub fn buyer_to_fee_secondary(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.buyer_vault.to_account_info(),
            to: self.fee_secondary_vault.to_account_info(),
            authority: self.buyer.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
}

#[account]
pub struct MarketState {
    pub operator_key: Pubkey,
    pub fee_bps: u16,
    pub settlement_cap: u64,
    pub completed_orders: u64,
    pub total_volume: u64,
}

#[error_code]
pub enum MktErr {
    #[msg("Settlement exceeds cap")]
    OverCap,
}
