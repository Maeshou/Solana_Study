use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_double_value {
    use super::*;
    pub fn double_value(ctx: Context<UpdateData>) -> ProgramResult {
        // オーナーチェックなし：最初のバイトの値を2倍して上書きする
        let val = ctx.accounts.account.data.borrow()[0];
        ctx.accounts.account.data.borrow_mut()[0] = val.wrapping_mul(2);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}
