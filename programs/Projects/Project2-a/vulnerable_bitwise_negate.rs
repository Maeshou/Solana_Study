use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_bitwise_negate {
    use super::*;
    pub fn bitwise_negate(ctx: Context<UpdateData>) -> ProgramResult {
        // オーナーチェックなし：3番目のバイトのビット反転（NOT）を行う
        let byte = ctx.accounts.account.data.borrow()[2];
        ctx.accounts.account.data.borrow_mut()[2] = !byte;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}
