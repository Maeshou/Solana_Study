use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Transfer, CpiContext, token};

declare_id!("Secu05255555555555555555555555555555555");

#[program]
pub mod case052 {
    use super::*;

    pub fn process_052(ctx: Context<Ctx052>) -> Result<()> {
        let from_t = ctx.accounts.acc_a.to_account_info();
let to_t = ctx.accounts.acc_b.to_account_info();
require!(from_t.key() != to_t.key(), ErrorCode::DuplicateAccount);
let cpi_acc = anchor_spl::token::Transfer {
    from: from_t.clone(),
    to: to_t.clone(),
    authority: ctx.accounts.acc_a.clone(),
};
anchor_spl::token::transfer(CpiContext::new(ctx.accounts.token_prog.to_account_info(), cpi_acc), 42)?;
msg!("Transferred 42 tokens via CPI");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx052<'info> {
    #[account(mut)]
    pub acc_a: AccountInfo<'info>,
    #[account(mut)]
    pub acc_b: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_prog: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Accounts must differ")]
    DuplicateAccount,
    #[msg("Insufficient resources")]
    InsufficientResources,
}
