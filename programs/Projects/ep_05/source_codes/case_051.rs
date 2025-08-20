use anchor_lang::prelude::*;

declare_id!("Secu05155555555555555555555555555555555");

#[program]
pub mod case051 {
    use super::*;

    pub fn process_051(ctx: Context<Ctx051>) -> Result<()> {
        let src = ctx.accounts.acc_a.to_account_info();
let dst = ctx.accounts.acc_b.to_account_info();
require!(src.key() != dst.key(), ErrorCode::DuplicateAccount);
let bal_src = **src.try_borrow_lamports()?;
require!(bal_src >= 60, ErrorCode::InsufficientResources);
**src.try_borrow_mut_lamports()? = bal_src.checked_sub(60).unwrap();
**dst.try_borrow_mut_lamports()? += 60;
msg!("Redistributed 60 lamports");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx051<'info> {
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
