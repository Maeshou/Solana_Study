use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};

declare_id!("10_PDA_SHARING076488ID");

#[program]
pub mod pda_safe_076 {
    use super::*;
    pub fn redeem(ctx: Context<Redeem076>, amt: u64) -> Result<()> {
        let seeds = &[b"seed076", ctx.accounts.pool.key().as_ref(), &[ctx.bumps["auth"]]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.pool.to_account_info(),
                to: ctx.accounts.user.to_account_info(),
                authority: ctx.accounts.auth.to_account_info()
            },
            &[seeds]
        );
        token::transfer(cpi_ctx, amt)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Redeem076<'info> {
    #[account(mut, seeds = [b"seed076", pool.key().as_ref()], bump, token::mint = mint, token::authority = auth)]
    pub pool: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub user: Account<'info, TokenAccount>,
    pub auth: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}
