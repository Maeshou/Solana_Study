use anchor_lang::prelude::*;

declare_id!("5Zn7GhR8BkPm2Qv1UdXf6SeZiNtOyAzPfLmH3JkLsRw");

#[program]
pub mod profile_points {
    use super::*;

    /// profile_a から profile_b へポイントを移動するが、
    /// 同一アカウントかどうかの検証が抜けている！
    pub fn transfer_points(
        ctx: Context<TransferPoints>,
        amount: u64,
    ) -> ProgramResult {
        let profile_a = &mut ctx.accounts.profile_a;
        let profile_b = &mut ctx.accounts.profile_b;

        // ❌ 本来はここで profile_a.key() != profile_b.key() をチェックすべき
        // require!(
        //     profile_a.key() != profile_b.key(),
        //     DuplicateAccounts
        // );

        // 残高チェック
        if profile_a.points < amount {
            return Err(ErrorCode::InsufficientPoints.into());
        }

        // 実際のポイント移動
        profile_a.points = profile_a.points.checked_sub(amount).unwrap();
        profile_b.points = profile_b.points.checked_add(amount).unwrap();

        msg!(
            "Transferred {} points from {} to {}",
            amount,
            profile_a.owner,
            profile_b.owner
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferPoints<'info> {
    /// 送信元プロファイル
    #[account(mut)]
    pub profile_a: Account<'info, UserProfile>,

    /// 送信先プロファイル
    #[account(mut)]
    pub profile_b: Account<'info, UserProfile>,

    /// 送金を指示するユーザー
    #[account(signer)]
    pub user: Signer<'info>,

    /// システムプログラム
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserProfile {
    /// プロファイル所有者
    pub owner: Pubkey,
    /// 保有ポイント
    pub points: u64,
}

#[error]
pub enum ErrorCode {
    #[msg("Not enough points in source profile.")]
    InsufficientPoints,
    #[msg("Source and destination must be different accounts.")]
    DuplicateAccounts,
}
