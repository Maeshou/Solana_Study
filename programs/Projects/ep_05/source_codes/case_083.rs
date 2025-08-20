use anchor_lang::prelude::*;
use solana_program::{system_instruction, program::invoke};

declare_id!("Secu08355555555555555555555555555555555");

#[program]
pub mod case083 {
    use super::*;

    pub fn process_083(ctx: Context<Ctx083>) -> Result<()> {
        let payer = ctx.accounts.acc_a.to_account_info();
let payee = ctx.accounts.acc_b.to_account_info();
require!(payer.key() != payee.key(), ErrorCode::DuplicateAccount);
let ix = solana_program::system_instruction::transfer(&payer.key(), &payee.key(), 250);
solana_program::program::invoke(&ix, &[payer.clone(), payee.clone()])?;
msg!("Invoked system transfer of 250 lamports");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx083<'info> {
    #[account(mut)]
    pub acc_a: AccountInfo<'info>,
    #[account(mut)]
    pub acc_b: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    
}

#[error_code]
pub enum ErrorCode {
    #[msg("Accounts must differ")]
    DuplicateAccount,
    #[msg("Insufficient resources")]
    InsufficientResources,
}
