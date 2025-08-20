use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer, TokenAccount, Token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf903mvTWf");

#[program]
pub mod release_tokens_903 {
    use super::*;

    pub fn release_tokens(ctx: Context<ReleaseTokens903>) -> Result<()> {
        let seeds = &[ctx.accounts.pool.destination.key().as_ref(), &[ctx.accounts.pool.bump]];
        let amount = ctx.accounts.vault.amount;
        transfer(ctx.accounts.token_program.to_account_info().with_signer(&[seeds]), amount)?;
        // Sysvar Clock から現在時刻を取得
        let clk = Clock::get()?;
        ctx.accounts.state.last_ts = clk.unix_timestamp as u64;
        msg!(
            "Case 903: transferred {} at ts {}",
            amount,
            ctx.accounts.state.last_ts
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReleaseTokens903<'info> {
    #[account(has_one=vault, has_one=destination, seeds=[destination.key().as_ref()], bump=pool.bump)]
    pub pool: Account<'info, Pool903>,
    #[account(mut)] pub vault: Account<'info, TokenAccount>,
    #[account(mut)] pub destination: Account<'info, TokenAccount>,
    #[account(mut, seeds=[b"state", pool.key().as_ref()], bump, payer=user, space=8+8)]
    pub state: Account<'info, State903>,
    #[account(signer)] pub user: Signer<'info>,
    #[account(address=token::ID)] pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct Pool903 {
    pub vault: Pubkey,
    pub destination: Pubkey,
    pub bump: u8,
}

#[account]
pub struct State903 {
    pub last_ts: u64,
}
