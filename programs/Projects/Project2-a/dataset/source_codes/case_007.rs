use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_example2 {
    use super::*;
    pub fn write_byte(ctx: Context<UpdateData>) -> ProgramResult {
        // Owner Checkをしていない
        ctx.accounts.account.data.borrow_mut()[1] = 255;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}