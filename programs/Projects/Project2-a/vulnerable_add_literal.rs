use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_add_literal {
    use super::*;
    pub fn add_literal(ctx: Context<UpdateData>) -> ProgramResult {
        // オーナーチェックなし：4番目のバイトに 3 と 7 の和（10）を設定する
        ctx.accounts.account.data.borrow_mut()[3] = 3 + 7;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}
