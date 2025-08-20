use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_sum_literals {
    use super::*;
    pub fn sum_literals(ctx: Context<UpdateData>) -> ProgramResult {
        // オーナーチェックなし：2番目のバイトに10と20の和（30）を設定する
        ctx.accounts.account.data.borrow_mut()[1] = 10 + 20;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}
