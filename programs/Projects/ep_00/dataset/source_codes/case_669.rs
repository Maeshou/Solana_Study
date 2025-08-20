use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};

declare_id!("8_ARBITRARY_CPI069882ID");

#[program]
pub mod safe_cpi_069 {
    use super::*;
    pub fn distribute(ctx: Context<Dist069>, amt: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.src.to_account_info(),
                to: ctx.accounts.dst.to_account_info(),
                authority: ctx.accounts.authority.to_account_info()
            }
        );
        token::transfer(cpi_ctx, amt)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Dist069<'info> {
    #[account(mut, token::mint = mint)]
    pub src: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub dst: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub mint: Account<'info, Mint>,
}
