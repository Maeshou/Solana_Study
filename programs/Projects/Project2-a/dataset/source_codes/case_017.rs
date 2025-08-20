use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_multiply {
    use super::*;
    pub fn multiply_byte(ctx: Context<UpdateData>) -> ProgramResult {
        // オーナーチェックなし：最初のバイトの値に3を掛けて更新する
        let val = ctx.accounts.account.data.borrow()[0];
        ctx.accounts.account.data.borrow_mut()[0] = val.wrapping_mul(3);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}
