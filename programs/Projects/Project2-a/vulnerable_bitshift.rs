use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_bitshift {
    use super::*;
    pub fn bitshift(ctx: Context<UpdateData>) -> ProgramResult {
        // オーナーチェックなし：2番目のバイトを左に1ビットシフトする
        let val = ctx.accounts.account.data.borrow()[1];
        ctx.accounts.account.data.borrow_mut()[1] = val << 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}
