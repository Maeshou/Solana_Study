use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqSeasonPass");

#[program]
pub mod nft_season_pass {
    use super::*;

    /// シーズンパスのレベルを更新し、報酬を付与する  
    /// (`season_pass_account` の owner チェックを一切行っていないため、  
    ///  攻撃者が他人のシーズンパスを指定して無制限にレベルを上げ、報酬を横取りできます)
    pub fn upgrade_season_pass(
        ctx: Context<UpgradeSeasonPass>,
        reward_amount: u64,   // レベルアップ報酬 (lamports)
    ) -> Result<()> {
        let pass_acc = &mut ctx.accounts.season_pass_account.to_account_info();
        let data     = &mut pass_acc.data.borrow_mut();

        // ―― レイアウト想定 ――
        // [0]       : u8   現在のシーズンパスレベル (0–255)
        // [1..9]    : u64  最終アップグレード時 UNIXタイムスタンプ
        if data.len() < 9 {
            return err!(ErrorCode::DataTooShort);
        }

        // 1) レベルを取得＆インクリメント（owner チェックなし）
        let current_level = data[0];
        let new_level = current_level.saturating_add(1);
        data[0] = new_level;

        // 2) タイムスタンプ更新
        let now = Clock::get()?.unix_timestamp as u64;
        data[1..9].copy_from_slice(&now.to_le_bytes());

        // 3) lamports をリワードプールからユーザーへ送金
        **ctx.accounts.reward_pool.to_account_info().lamports.borrow_mut() = 
            ctx.accounts.reward_pool.to_account_info().lamports()
            .saturating_sub(reward_amount);
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() += reward_amount;

        msg!(
            "Season pass {} upgraded: {} → {} at {}, rewarded {} lamports",
            pass_acc.key(),
            current_level,
            new_level,
            now,
            reward_amount
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpgradeSeasonPass<'info> {
    /// CHECK: owner == program_id の検証をまったく行っていない AccountInfo
    #[account(mut)]
    pub season_pass_account: AccountInfo<'info>,

    /// 報酬プールアカウント（owner チェックなし）
    #[account(mut)]
    pub reward_pool:         AccountInfo<'info>,

    /// アップグレードを行うユーザー（署名のみ検証）
    #[account(mut)]
    pub user:                Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが想定より短いです")]
    DataTooShort,
}
