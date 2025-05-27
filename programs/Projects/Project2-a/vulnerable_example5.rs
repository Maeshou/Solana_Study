use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_example5 {
    use super::*;
    pub fn write_local_value(ctx: Context<UpdateData>) -> ProgramResult {
        let value: u8 = 100;
        // チェックを省略してローカル変数の値をそのまま書き込み
        ctx.accounts.account.data.borrow_mut()[0] = value;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}