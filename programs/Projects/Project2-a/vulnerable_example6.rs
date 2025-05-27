use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_example6 {
    use super::*;
    pub fn write_at_fourth_byte(ctx: Context<UpdateData>) -> ProgramResult {
        // オーナーチェックなしで直接書き込み
        ctx.accounts.account.data.borrow_mut()[3] = 7;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}