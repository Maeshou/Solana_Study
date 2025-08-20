use anchor_lang::prelude::*;

declare_id!("Secu00655555555555555555555555555555555");

#[program]
pub mod case006 {
    use super::*;

    pub fn process_006(ctx: Context<Ctx006>) -> Result<()> {
        let src = ctx.accounts.acc_a.to_account_info();
let dst = ctx.accounts.acc_b.to_account_info();
require!(src.key() != dst.key(), ErrorCode::DuplicateAccount);
let bal_src = **src.try_borrow_lamports()?;
require!(bal_src >= 110, ErrorCode::InsufficientResources);
**src.try_borrow_mut_lamports()? = bal_src.checked_sub(110).unwrap();
**dst.try_borrow_mut_lamports()? += 110;
msg!("Redistributed 110 lamports");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx006<'info> {
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
