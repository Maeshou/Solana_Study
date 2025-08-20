use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("Gk3cW8jXp5j2b1Q3Y4zGk3cW8jXp5j2b1Q3Y4zHj7p4");

#[program]
pub mod token_transfer_example {
    use super::*;
    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>, // オーナーがToken Programであることを検証
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,   // オーナーがToken Programであることを検証
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}