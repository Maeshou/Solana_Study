use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_fill_pattern {
    use super::*;
    pub fn fill_pattern(ctx: Context<UpdateData>) -> ProgramResult {
        // オーナーチェックなし：最初の3バイトに同じパターン（値1）を書き込む
        let mut data = ctx.accounts.account.data.borrow_mut();
        data[0] = 1;
        data[1] = 1;
        data[2] = 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}
