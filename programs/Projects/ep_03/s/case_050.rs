use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgTreasureMultiFn01");

#[program]
pub mod treasure_hunt {
    use super::*;

    /// 宝箱報酬を複数の内部関数で処理する例
    pub fn open_treasure(ctx: Context<OpenTreasure>) -> Result<()> {
        // 1. 報酬額を取得
        let amount = get_reward_amount(&ctx.accounts.config);

        // 2. Lamports を移動
        transfer_reward(&ctx.accounts.reward_vault.to_account_info(), &ctx.accounts.user.to_account_info(), amount)?;

        Ok(())
    }
}

/// 設定アカウントから報酬額を返すヘルパー関数
fn get_reward_amount(config: &Account<TreasureConfig>) -> u64 {
    config.reward_amount
}

/// Vault からユーザーへ直接 Lamports を移動するヘルパー関数
fn transfer_reward(vault: &AccountInfo, user: &AccountInfo, amount: u64) -> Result<()> {
    **vault.lamports.borrow_mut() -= amount;
    **user.lamports.borrow_mut() += amount;
    Ok(())
}

#[derive(Accounts)]
pub struct OpenTreasure<'info> {
    /// 宝箱アカウント（フラグは操作しない）
    #[account(mut)]
    pub treasure_account: Account<'info, TreasureAccount>,

    /// 報酬プールアカウント
    #[account(mut)]
    pub reward_vault: AccountInfo<'info>,

    /// 報酬を受け取るユーザー
    #[account(mut)]
    pub user: Signer<'info>,

    /// 報酬量設定アカウント
    pub config: Account<'info, TreasureConfig>,
}

#[account]
pub struct TreasureAccount {
    /// 本来この宝箱を所有するはずのユーザー Pubkey
    pub owner: Pubkey,
    /// フラグは残るが used しない
    pub opened: bool,
}

#[account]
pub struct TreasureConfig {
    /// 支払い時に移動する Lamports 量
    pub reward_amount: u64,
}
