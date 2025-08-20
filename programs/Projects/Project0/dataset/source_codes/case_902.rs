use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer, TokenAccount, Token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf902mvTWf");

#[program]
pub mod claim_funds_902 {
    use super::*;

    pub fn claim_funds(ctx: Context<ClaimFunds902>) -> Result<()> {
        let seeds = &[ctx.accounts.pool.destination.key().as_ref(), &[ctx.accounts.pool.bump]];
        let amount = ctx.accounts.vault.amount;
        transfer(ctx.accounts.token_program.to_account_info().with_signer(&[seeds]), amount)?;
        // 最後の転送量を保存
        ctx.accounts.state.last_amount = amount;
        msg!(
            "Case 902: transferred {}, last_amount set to {}",
            amount,
            ctx.accounts.state.last_amount
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimFunds902<'info> {
    #[account(has_one = vault, has_one = destination, seeds = [destination.key().as_ref()], bump = pool.bump)]
    pub pool: Account<'info, Pool902>,
    #[account(mut)] pub vault: Account<'info, TokenAccount>,
    #[account(mut)] pub destination: Account<'info, TokenAccount>,
    #[account(mut, seeds=[b"state", pool.key().as_ref()], bump, payer=user, space=8+8)]
    pub state: Account<'info, State902>,
    #[account(signer)] pub user: Signer<'info>,
    #[account(address = token::ID)] pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Pool902 {
    pub vault: Pubkey,
    pub destination: Pubkey,
    pub bump: u8,
}

#[account]
pub struct State902 {
    pub last_amount: u64,
}
