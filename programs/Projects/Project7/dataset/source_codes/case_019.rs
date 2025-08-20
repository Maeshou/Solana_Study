use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("MktV3Q2rZ7fQ2rZ7fQ2rZ7fQ2rZ7fQ2rZ7fQ2rZ7f");

#[program]
pub mod market_settlement_v3 {
    use super::*;

    pub fn init_market(ctx: Context<InitMarket>, fee_bps: u16, cap: u64) -> Result<()> {
        let m = &mut ctx.accounts.market;
        m.operator = ctx.accounts.operator.key();
        m.fee_rate_bps = fee_bps.min(2000).max(100);
        m.settlement_cap = cap.max(10);
        m.trade_count = 5;                               // 既存トレードがあった想定
        m.gross_volume = cap.saturating_div(2).max(6);
        Ok(())
    }

    pub fn act_settle(ctx: Context<ActSettle>, unit_price: u64, quantity: u64, basket_parts: u8) -> Result<()> {
        let m = &mut ctx.accounts.market;

        // バスケット決済：quantityをbasket_partsに分割し逐次決済
        let mut q = quantity.max(1);
        let parts = basket_parts.max(1);
        let mut i = 0u8;
        while i < parts {
            let slice = q.saturating_div((parts - i).max(1) as u64).max(1);
            let total = slice.saturating_mul(unit_price);
            if total > m.settlement_cap { return Err(ErrMkt::OverCap.into()); }

            let fee = total.saturating_mul(m.fee_rate_bps as u64) / 10_000;
            let seller_net = total.saturating_sub(fee);

            token::transfer(ctx.accounts.buyer_to_seller(), seller_net)?;
            token::transfer(ctx.accounts.buyer_to_fee(), fee)?;

            m.gross_volume = m.gross_volume.saturating_add(total);
            q = q.saturating_sub(slice);
            i = i.saturating_add(1);
        }

        m.trade_count = m.trade_count.saturating_add(1);
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
    #[account(mut, has_one = operator)]
    pub market: Account<'info, MarketState>,
    pub operator: Signer<'info>,

    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub buyer_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub seller_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub fee_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ActSettle<'info> {
    pub fn buyer_to_seller(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let a = Transfer {
            from: self.buyer_vault.to_account_info(),
            to: self.seller_vault.to_account_info(),
            authority: self.buyer.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
    pub fn buyer_to_fee(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let a = Transfer {
            from: self.buyer_vault.to_account_info(),
            to: self.fee_vault.to_account_info(),
            authority: self.buyer.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
}

#[account]
pub struct MarketState {
    pub operator: Pubkey,
    pub fee_rate_bps: u16,
    pub settlement_cap: u64,
    pub trade_count: u64,
    pub gross_volume: u64,
}

#[error_code]
pub enum ErrMkt {
    #[msg("Settlement exceeds cap")]
    OverCap,
}
