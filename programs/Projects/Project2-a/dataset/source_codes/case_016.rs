use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_modulo {
    use super::*;
    pub fn modulo_value(ctx: Context<UpdateData>) -> ProgramResult {
        // オーナーチェックなし：最初のバイトの値を5で割った余りをその位置に再設定する
        let val = ctx.accounts.account.data.borrow()[0];
        ctx.accounts.account.data.borrow_mut()[0] = val % 5;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}
