use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};

declare_id!("10_PDA_SHARING039617ID");

#[program]
pub mod pda_safe_039 {
    use super::*;
    pub fn redeem(ctx: Context<Redeem039>, amt: u64) -> Result<()> {
        let seeds = &[b"seed039", ctx.accounts.pool.key().as_ref(), &[ctx.bumps["auth"]]];
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
pub struct Redeem039<'info> {
    #[account(mut, seeds = [b"seed039", pool.key().as_ref()], bump, token::mint = mint, token::authority = auth)]
    pub pool: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub user: Account<'info, TokenAccount>,
    pub auth: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}
