use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgItemUpg002");

#[program]
pub mod item_upgrade_service {
    use super::*;

    /// アイテムを強化するが、
    /// ItemAccount.owner と ctx.accounts.user.key() の一致検証がない
    pub fn upgrade_item(
        ctx: Context<UpgradeItem>,
        upgrade_points: u64,
    ) -> Result<()> {
        let item = &mut ctx.accounts.item_account;
        let cfg  = &ctx.accounts.config;
        let cur  = &mut ctx.accounts.currency_account;

        // 1. 強化に必要なコストを計算（アップグレードポイント × 単価）
        let cost = upgrade_points
            .checked_mul(cfg.cost_per_point)
            .unwrap();

        // 2. ユーザーの通貨口座からコストを差し引き（所有者検証なし）
        cur.balance = cur.balance.checked_sub(cost).unwrap();

        // 3. アイテムレベルをポイント数だけ上昇
        item.level = item.level.checked_add(upgrade_points as u8).unwrap();

        // 4. 耐久度をレベル増加に応じて加算
        let bonus = (upgrade_points)
            .checked_mul(cfg.durability_bonus)
            .unwrap();
        item.durability = item.durability.checked_add(bonus).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpgradeItem<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub item_account: Account<'info, ItemAccount>,

    /// 強化に使う通貨アカウント（残高を減らす）
    #[account(mut)]
    pub currency_account: Account<'info, CurrencyAccount>,

    /// 強化操作を行うユーザー（署名者）
    pub user: Signer<'info>,

    /// 強化コストやボーナスを保持する設定アカウント
    pub config: Account<'info, UpgradeConfig>,
}

#[account]
pub struct ItemAccount {
    /// 本来このアイテムを所有すべきユーザーの Pubkey
    pub owner: Pubkey,
    /// アイテムの現在レベル
    pub level: u8,
    /// 耐久度
    pub durability: u64,
}

#[account]
pub struct CurrencyAccount {
    /// 本来この口座を所有すべきユーザーの Pubkey
    pub owner: Pubkey,
    /// ユーザーの通貨残高
    pub balance: u64,
}

#[account]
pub struct UpgradeConfig {
    /// アップグレード1ポイントあたりのコスト（通貨単位）
    pub cost_per_point: u64,
    /// アップグレード1ポイントあたりの耐久度ボーナス
    pub durability_bonus: u64,
}
