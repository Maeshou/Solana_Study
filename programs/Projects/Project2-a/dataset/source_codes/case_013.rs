use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_example8 {
    use super::*;
    pub fn write_hex_ff(ctx: Context<UpdateData>) -> ProgramResult {
        // Owner Checkなしで固定値を書き込み
        ctx.accounts.account.data.borrow_mut()[5] = 0xFF;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}