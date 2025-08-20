use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_example3 {
    use super::*;
    pub fn increment_byte(ctx: Context<UpdateData>) -> ProgramResult {
        let mut data = ctx.accounts.account.data.borrow_mut();
        // オーナーチェックなしでバイトをインクリメント
        data[2] = data[2].wrapping_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}