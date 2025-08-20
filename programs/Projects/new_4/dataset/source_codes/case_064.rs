// NFTゲーム関連 misinitパターン v8 改訂版2：push や bool を削減し、代わりに insert や u8ステータスを利用

// 1. クラフティング（NFT合成）＋クラフト履歴
use anchor_lang::prelude::*;
declare_id!("CFVG1Craft1111222233334444AAAA5555");

#[program]
pub mod misinit_crafting_v8 {
    use super::*;

    pub fn init_craft(
        ctx: Context<InitCraft>,
        recipe_id: u32,
    ) -> Result<()> {
        let craft = &mut ctx.accounts.craft_account;
        craft.recipe_id = recipe_id;
        craft.status = 0; // 0=未実行, 1=成功, 2=失敗
        Ok(())
    }

    pub fn execute_craft(
        ctx: Context<InitCraft>,
        outcome: u8,
    ) -> Result<()> {
        let craft = &mut ctx.accounts.craft_account;
        craft.status = outcome;
        Ok(())
    }

    pub fn log_craft(
        ctx: Context<InitCraft>,
        used: Vec<u32>,
    ) -> Result<()> {
        let log = &mut ctx.accounts.craft_log;
        // 最新が先頭
        log.materials.insert(0, used);
        // 10件保持
        if log.materials.len() > 10 { log.materials.pop(); }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCraft<'info> {
    #[account(init, payer = user, space = 8 + 4 + 1)]
    pub craft_account: Account<'info, CraftData>,
    #[account(mut)] pub craft_log: Account<'info, CraftLog>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CraftData { pub recipe_id: u32, pub status: u8 }
#[account]
pub struct CraftLog { pub materials: Vec<Vec<u32>> }
