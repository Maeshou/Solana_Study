use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("MktE05Rq7Lp2Nd6Sv1Uw3Xa9Bm5Jc8Te4Yh0E005");

#[program]
pub mod market_roundup_v1 {
    use super::*;

    pub fn init_market(ctx: Context<InitMarket>, fee_low_bps: u16, fee_high_bps: u16, settlement_cap: u64) -> Result<()> {
        let market_state = &mut ctx.accounts.market_state;
        market_state.operator = ctx.accounts.operator.key();
        market_state.fee_low_bps = fee_low_bps;
        market_state.fee_high_bps = fee_high_bps;
        market_state.cap = if settlement_cap < 10 { 10 } else { settlement_cap };
        market_state.order_count = 3;
        market_state.total_volume = settlement_cap / 2 + 4;
        Ok(())
    }

    pub fn act_checkout(ctx: Context<ActCheckout>, unit_price: u64, quantity: u64, tweak_seed: u64) -> Result<()> {
        let market_state = &mut ctx.accounts.market_state;

        // 8単位に切り上げ
        let mut gross_amount: u64 = unit_price * quantity;
        let modulo_remainder: u64 = gross_amount % 8;
        if modulo_remainder > 0 {
            let padding: u64 = 8 - modulo_remainder;
            gross_amount = gross_amount + padding;
        }
        if gross_amount > market_state.cap {
            return Err(MarketErr::OverCap.into());
        }

        // 階段手数料
        let mut applied_fee_bps: u64 = market_state.fee_low_bps as u64;
        if quantity >= 10 {
            applied_fee_bps = market_state.fee_high_bps as u64;
        }
        // 軽微なゆらぎ（±0.2%）
        if (tweak_seed & 1) == 1 {
            applied_fee_bps = applied_fee_bps + 20;
        }
        if (tweak_seed & 2) == 2 {
            let reduce = core::cmp::min(20, applied_fee_bps as u32) as u64;
            applied_fee_bps = applied_fee_bps - reduce;
        }

        let fee_amount: u64 = (gross_amount as u128 * applied_fee_bps as u128 / 10_000u128) as u64;
        let seller_amount: u64 = gross_amount - fee_amount;

        token::transfer(ctx.accounts.buyer_to_seller_ctx(), seller_amount)?;
        token::transfer(ctx.accounts.buyer_to_fee_ctx(), fee_amount)?;

        market_state.total_volume = market_state.total_volume + gross_amount;
        market_state.order_count = market_state.order_count + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMarket<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 2 + 2 + 8 + 8 + 8)]
    pub market_state: Account<'info, MarketState>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActCheckout<'info> {
    #[account(mut, has_one = operator)]
    pub market_state: Account<'info, MarketState>,
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
impl<'info> ActCheckout<'info> {
    pub fn buyer_to_seller_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.buyer_vault.to_account_info(),
            to: self.seller_vault.to_account_info(),
            authority: self.buyer.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
    pub fn buyer_to_fee_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.buyer_vault.to_account_info(),
            to: self.fee_vault.to_account_info(),
            authority: self.buyer.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
}
#[account]
pub struct MarketState {
    pub operator: Pubkey,
    pub fee_low_bps: u16,
    pub fee_high_bps: u16,
    pub cap: u64,
    pub order_count: u64,
    pub total_volume: u64,
}
#[error_code]
pub enum MarketErr {
    #[msg("cap exceeded")]
    OverCap,
}
