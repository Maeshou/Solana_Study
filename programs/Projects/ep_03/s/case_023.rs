use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgInvExp0001");

#[program]
pub mod game_inventory {
    use super::*;

    /// インベントリのスロット数を拡張し、通貨を消費するが、
    /// inventory_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn expand_inventory(
        ctx: Context<ExpandInventory>,
        slot_count: u64,
    ) -> Result<()> {
        let inv = &mut ctx.accounts.inventory_account;
        let cfg = &ctx.accounts.config;
        let cur = &mut ctx.accounts.currency_account;

        // 1. 必要コストを計算 (slot_count × cost_per_slot)
        let total_cost = slot_count
            .checked_mul(cfg.cost_per_slot)
            .unwrap();

        // 2. ユーザーの通貨アカウント残高からコストを差し引き
        //    本来は cur.owner と ctx.accounts.user.key() の一致を検証すべき
        cur.balance = cur.balance.checked_sub(total_cost).unwrap();

        // 3. インベントリのスロット数を増加
        inv.slot_count = inv.slot_count.checked_add(slot_count).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExpandInventory<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して照合チェックを行うべき
    pub inventory_account: Account<'info, InventoryAccount>,

    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して照合チェックを行うべき
    pub currency_account: Account<'info, CurrencyAccount>,

    /// インベントリ拡張を実行するユーザー（署名者）
    pub user: Signer<'info>,

    /// 拡張コスト設定を保持するアカウント
    pub config: Account<'info, GameConfig>,
}

#[account]
pub struct InventoryAccount {
    /// このインベントリアカウントを所有すべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 現在のスロット数
    pub slot_count: u64,
}

#[account]
pub struct CurrencyAccount {
    /// この通貨アカウントを所有すべきユーザーの Pubkey
    pub owner: Pubkey,
    /// ユーザーの保有通貨残高
    pub balance: u64,
}

#[account]
pub struct GameConfig {
    /// スロット1つあたりのコスト（通貨単位）
    pub cost_per_slot: u64,
}
