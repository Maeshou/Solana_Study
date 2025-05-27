use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_example1 {
    use super::*;
    pub fn write_fixed_value(ctx: Context<UpdateData>) -> ProgramResult {
        // Owner Checkを行っていない（脆弱性）
        ctx.accounts.account.data.borrow_mut()[0] = 42;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}