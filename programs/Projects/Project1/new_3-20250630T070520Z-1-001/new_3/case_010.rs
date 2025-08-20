use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgCraftSvc01");

#[program]
pub mod crafting_service {
    use super::*;

    /// リソースを消費してアイテムを作成するが、
    /// resource_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn craft_item(ctx: Context<CraftItem>) -> Result<()> {
        let res = &mut ctx.accounts.resource_account;

        // 消費リソース量を定義
        let wood_needed: u64 = 10;
        let iron_needed: u64 = 5;

        // ↓ 本来は res.owner と ctx.accounts.user.key() の一致を検証すべき
        res.wood = res.wood.checked_sub(wood_needed).unwrap();
        res.iron = res.iron.checked_sub(iron_needed).unwrap();

        // 作成済アイテム数をインクリメント
        res.crafted_count = res.crafted_count.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CraftItem<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者チェックを行うべき
    pub resource_account: Account<'info, ResourceAccount>,
    /// リソース所有者としての署名チェックが必要
    pub user: Signer<'info>,
}

#[account]
pub struct ResourceAccount {
    /// このリソースアカウントを所有するユーザーの Pubkey
    pub owner: Pubkey,
    /// 保有する木材の量
    pub wood: u64,
    /// 保有する鉄の量
    pub iron: u64,
    /// これまでに作成したアイテム数
    pub crafted_count: u64,
}
