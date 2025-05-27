use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_set_const {
    use super::*;
    pub fn set_const(ctx: Context<UpdateData>) -> ProgramResult {
        // オーナーチェックなし：先頭バイトに42を書き込む
        ctx.accounts.account.data.borrow_mut()[0] = 42;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}
