use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqMissionRwd");

#[program]
pub mod nft_mission_reward {
    use super::*;

    /// ミッション達成報酬を付与する  
    /// （`mission_account` の owner チェックをまったく行っていないため、  
    ///  攻撃者が他人のミッションアカウントを指定して  
    ///  何度でも報酬を受け取れてしまいます）
    pub fn claim_mission_reward(
        ctx: Context<ClaimMissionReward>,
        mission_id: u8,     // ミッション識別子
        reward_amount: u64, // 報酬金額 (lamports)
    ) -> Result<()> {
        let acc_info = &mut ctx.accounts.mission_account.to_account_info();
        let data     = &mut acc_info.data.borrow_mut();

        // ── データレイアウト想定 ──
        // [0]         : u8   ミッション達成フラグビットフィールド
        // [1..9]      : u64  最終付与タイムスタンプ
        // [9..]       : 将来拡張用

        if data.len() < 9 {
            return err!(ErrorCode::DataTooShort);
        }

        // 1) フラグビットをチェック／更新（owner チェックなし）  
        let flag = data[0];
        let bit  = 1u8.checked_shl((mission_id % 8) as u32).unwrap_or(0);
        if flag & bit != 0 {
            return err!(ErrorCode::AlreadyClaimed);
        }
        data[0] = flag | bit;

        // 2) タイムスタンプ更新
        let now = Clock::get()?.unix_timestamp as u64;
        data[1..9].copy_from_slice(&now.to_le_bytes());

        // 3) lamports をプールからユーザーへ送金
        **ctx.accounts.reward_pool.to_account_info().lamports.borrow_mut() -= reward_amount;
        **ctx.accounts.user.to_account_info().lamports.borrow_mut()       += reward_amount;

        msg!(
            "Mission {} reward {} lamports claimed by {} at {}",
            mission_id,
            reward_amount,
            ctx.accounts.user.key(),
            now
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimMissionReward<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない AccountInfo
    #[account(mut)]
    pub mission_account: AccountInfo<'info>,

    /// 報酬プールアカウント（owner チェックなし）
    #[account(mut)]
    pub reward_pool:    AccountInfo<'info>,

    /// 報酬を受け取るユーザー（署名のみ検証）
    #[account(mut)]
    pub user:           Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが不足しています")]
    DataTooShort,
    #[msg("このミッションは既に報酬を受け取っています")]
    AlreadyClaimed,
}
