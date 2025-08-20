use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("M4rketSeTTL3Fe311111111111111111111111111");

#[program]
pub mod market_settlement {
    use super::*;
    pub fn init_market(ctx: Context<InitMarket>, fee_bps: u16, settle_cap: u64) -> Result<()> {
        let m = &mut ctx.accounts.market;
        m.operator = ctx.accounts.operator.key();
        m.fee_bps = fee_bps.min(1500);
        m.settle_cap = settle_cap;
        m.gross_volume = 0;
        m.settlements = 0;
        Ok(())
    }

    pub fn act_settle(ctx: Context<ActSettle>, price: u64, quantity: u64) -> Result<()> {
        let m = &mut ctx.accounts.market;
        let mut total = 0u64;
        for _ in 0..quantity {
            total = total.saturating_add(price);
        }

        if total > m.settle_cap {
            return Err(ErrorCode::OverCap.into());
        }

        let fee = total.saturating_mul(m.fee_bps as u64) / 10_000;
        let seller_net = total.saturating_sub(fee);

        // buyer -> seller
        token::transfer(ctx.accounts.pay_buyer_to_seller(), seller_net)?;
        // buyer -> fee_vault
        token::transfer(ctx.accounts.pay_buyer_to_fee(), fee)?;

        m.gross_volume = m.gross_volume.saturating_add(total);
        m.settlements = m.settlements.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMarket<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 2 + 8 + 8 + 8)]
    pub market: Account<'info, Market>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActSettle<'info> {
    #[account(mut, has_one = operator)]
    pub market: Account<'info, Market>,
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
    pub fn pay_buyer_to_seller(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let accs = Transfer {
            from: self.buyer_vault.to_account_info(),
            to: self.seller_vault.to_account_info(),
            authority: self.buyer.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accs)
    }
    pub fn pay_buyer_to_fee(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let accs = Transfer {
            from: self.buyer_vault.to_account_info(),
            to: self.fee_vault.to_account_info(),
            authority: self.buyer.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accs)
    }
}

#[account]
pub struct Market {
    pub operator: Pubkey,
    pub fee_bps: u16,
    pub settle_cap: u64,
    pub gross_volume: u64,
    pub settlements: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Settlement exceeds cap")]
    OverCap,
}
