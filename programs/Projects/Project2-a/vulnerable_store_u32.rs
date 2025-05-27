use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_store_u32 {
    use super::*;
    pub fn store_u32(ctx: Context<UpdateData>) -> ProgramResult {
        // オーナーチェックなし：定数0xDEADBEEFをリトルエンディアン形式で先頭4バイトに格納する
        let value: u32 = 0xDEADBEEF;
        let mut data = ctx.accounts.account.data.borrow_mut();
        data[0] = (value & 0xFF) as u8;
        data[1] = ((value >> 8) & 0xFF) as u8;
        data[2] = ((value >> 16) & 0xFF) as u8;
        data[3] = ((value >> 24) & 0xFF) as u8;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}
