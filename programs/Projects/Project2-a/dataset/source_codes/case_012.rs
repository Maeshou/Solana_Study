use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_example7 {
    use super::*;
    pub fn write_at_fifth_byte(ctx: Context<UpdateData>) -> ProgramResult {
        // 期待されるオーナーチェックを省略して書き込み
        ctx.accounts.account.data.borrow_mut()[4] = 77;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}