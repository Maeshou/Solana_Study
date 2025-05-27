use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token, Transfer, CpiContext};

declare_id!("Prog04311111111111111111111111111111111");

#[program]
pub mod case043 {
    use super::*;

    pub fn split_account(ctx: Context<Ctx043>, amount: u64) -> Result<()> {
        // 脆弱性: 2 つの同型トークンアカウント比較なし
        let cpi_accounts = Transfer {
            from: ctx.accounts.from_token.to_account_info(),
            to: ctx.accounts.to_token.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx043<'info> {
    #[account(mut)]
    pub from_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_token: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
