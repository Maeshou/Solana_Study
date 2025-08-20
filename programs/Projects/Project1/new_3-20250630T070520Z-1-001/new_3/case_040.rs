use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgReprSvc001");

#[program]
pub mod repair_service {
    use super::*;

    /// 耐久度を回復し、費用を徴収するが、
    /// item_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn repair_item(ctx: Context<RepairItem>, restore_amount: u8) -> Result<()> {
        let item = &mut ctx.accounts.item_account;
        let cfg = &ctx.accounts.config;

        // 1. 回復量分だけ耐久度を加算
        item.durability = item
            .durability
            .checked_add(restore_amount as u64)
            .unwrap();

        // 2. 回復コストを計算 (ポイント単価 × 回復量)
        let cost = (restore_amount as u64)
            .checked_mul(cfg.cost_per_point)
            .unwrap();

        // 3. ユーザーから運営口座へ Lamports を移動
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() -= cost;
        **ctx.accounts.treasury.to_account_info().lamports.borrow_mut() += cost;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RepairItem<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して照合チェックを入れるべき
    pub item_account: Account<'info, ItemAccount>,

    /// 回復を実行するユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,

    /// 徴収した費用を受け取る口座
    #[account(mut)]
    pub treasury: AccountInfo<'info>,

    /// 回復コスト設定アカウント
    pub config: Account<'info, RepairConfig>,
}

#[account]
pub struct ItemAccount {
    /// 本来このアイテムを所有するユーザーの Pubkey
    pub owner: Pubkey,
    /// 現在の耐久度
    pub durability: u64,
}

#[account]
pub struct RepairConfig {
    /// 回復 1 ポイントあたりのコスト（Lamports）
    pub cost_per_point: u64,
}
