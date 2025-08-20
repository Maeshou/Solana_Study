use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_example9 {
    use super::*;
    pub fn write_hex_aa(ctx: Context<UpdateData>) -> ProgramResult {
        // チェックを省略して書き込み
        ctx.accounts.account.data.borrow_mut()[6] = 0xAA;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}