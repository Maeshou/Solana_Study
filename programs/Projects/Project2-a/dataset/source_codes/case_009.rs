use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_example4 {
    use super::*;
    pub fn write_two_bytes(ctx: Context<UpdateData>) -> ProgramResult {
        let mut data = ctx.accounts.account.data.borrow_mut();
        // オーナーチェックなし
        data[0] = 0;
        data[1] = 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}