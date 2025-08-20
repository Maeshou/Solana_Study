// 7. 登録管理＋解除
use anchor_lang::prelude::*;

declare_id!("Reg77777777777777777777777777777777");

#[program]
pub mod reinit_registry_v2 {
    use super::*;

    // アイテムを追加
    pub fn register_item(
        ctx: Context<ModifyRegistry>,
        key: String,
        value: u64,
    ) -> Result<()> {
        let reg = &mut ctx.accounts.registry;
        reg.key = key;
        reg.value = value;
        reg.count = reg.count + 1;
        Ok(())
    }

    // アイテムを削除
    pub fn unregister_item(
        ctx: Context<ModifyRegistry>,
    ) -> Result<()> {
        let reg = &mut ctx.accounts.registry;
        reg.key.clear();
        reg.value = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyRegistry<'info> {
    #[account(mut)]
    pub registry: Account<'info, RegistryData>,
    /// CHECK: カウンタ用、適切な初期化なし
    #[account(mut)]
    pub event_counter: AccountInfo<'info>,
}

#[account]
pub struct RegistryData {
    pub key: String,
    pub value: u64,
    pub count: u64,
}
