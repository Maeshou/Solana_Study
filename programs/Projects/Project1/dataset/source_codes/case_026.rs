use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA12mvTWf");

#[program]
pub mod bitflag_manager_003 {
    use super::*;

    pub fn set_flag_at(ctx: Context<Ctx003>, index: u8) -> Result<()> {
        let shift = 1u8 << (index % 8); // 0〜7に制限（ifなし）
        let current = ctx.accounts.storage.flag_register;
        ctx.accounts.storage.flag_register = current | shift;
        Ok(())
    }

    pub fn clear_flag_at(ctx: Context<Ctx003>, index: u8) -> Result<()> {
        let mask = !(1u8 << (index % 8)); // 0〜7制限（分岐なし）
        let current = ctx.accounts.storage.flag_register;
        ctx.accounts.storage.flag_register = current & mask;
        Ok(())
    }

    pub fn display_flags(ctx: Context<Ctx003>) -> Result<()> {
        let flags = ctx.accounts.storage.flag_register;
        msg!("Current Flags: {:#010b}", flags); // 8ビット表示（例: 0b01011010）
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx003<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage003>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage003 {
    pub authority: Pubkey,
    pub flag_register: u8, // 各bitが状態フラグを表す
}
