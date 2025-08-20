use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};

declare_id!("TokenDispt77777777777777777777777777777777");

#[program]
pub mod token_dispatch {
    use super::*;

    pub fn send(ctx: Context<Send>, amount: u64) -> Result<()> {
        let cpi = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.from.clone(),
                to: ctx.accounts.to.clone(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        anchor_spl::token::transfer(cpi, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Send<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
