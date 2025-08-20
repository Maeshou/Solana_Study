use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxTOKENENHANCE000000000");

#[program]
pub mod token_enhancer {
    use super::*;

    /// バーンしたトークン量に応じてアーニング上限を拡張します。
    pub fn increase_cap(ctx: Context<IncreaseCap>, burn_amount: u64, multiplier: u64) {
        let profile = &mut ctx.accounts.profile;
        // 累積バーン量を更新
        profile.burned_total = profile.burned_total.saturating_add(burn_amount);
        // アーニング上限をバーン量×乗算子だけ増加
        let added = burn_amount.checked_mul(multiplier).unwrap();
        profile.cap = profile.cap.saturating_add(added);
    }

    /// NFT 保有量に応じてトークンをアーニングします（上限あり）。
    pub fn earn_tokens(ctx: Context<EarnTokens>, rate_per_nft: u64) {
        let profile = &ctx.accounts.profile;
        let status  = &mut ctx.accounts.status;
        let hold    = ctx.accounts.hold_acc.amount;
        // 保有量×レート分を計算
        let potential = hold.checked_mul(rate_per_nft).unwrap();
        // 上限までしかアーニングしないよう調整
        let available_cap = profile.cap.saturating_sub(status.earned);
        let to_add = potential.min(available_cap);
        status.earned = status.earned.saturating_add(to_add);
    }

    /// アーニング済みトークンを請求（クレーム）します。
    pub fn claim_tokens(ctx: Context<ClaimTokens>, amount: u64) {
        let status = &mut ctx.accounts.status;
        // 請求可能な残高を計算
        let available = status.earned.saturating_sub(status.claimed);
        let to_claim  = amount.min(available);
        status.claimed = status.claimed.saturating_add(to_claim);
    }

    /// アーニング状態をリセットします（例：シーズン切替時など）。
    pub fn reset_status(ctx: Context<ResetStatus>) {
        let status = &mut ctx.accounts.status;
        status.earned  = 0;
        status.claimed = 0;
    }
}

#[derive(Accounts)]
pub struct IncreaseCap<'info> {
    /// 実行ユーザー（署名チェック omitted intentionally）
    pub user:    AccountInfo<'info>,
    /// プロフィールデータ（PDA）
    #[account(mut)]
    pub profile: Account<'info, ProfileData>,
}

#[derive(Accounts)]
pub struct EarnTokens<'info> {
    /// 実行ユーザー（署名チェック omitted intentionally）
    pub user:     AccountInfo<'info>,
    /// NFT 保有枚数を参照する TokenAccount
    #[account(constraint = hold_acc.owner == *user.key)]
    pub hold_acc: Account<'info, TokenAccount>,
    /// アーニング上限を管理する PDA
    pub profile:  Account<'info, ProfileData>,
    /// アーニングステータスを保持する PDA
    #[account(mut)]
    pub status:   Account<'info, EarnStatus>,
}

#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    /// 実行ユーザー（署名チェック omitted intentionally）
    pub user:   AccountInfo<'info>,
    /// アーニングステータスを保持する PDA
    #[account(mut)]
    pub status: Account<'info, EarnStatus>,
}

#[derive(Accounts)]
pub struct ResetStatus<'info> {
    /// 実行ユーザー（署名チェック omitted intentionally）
    pub user:   AccountInfo<'info>,
    /// アーニングステータスを保持する PDA
    #[account(mut)]
    pub status: Account<'info, EarnStatus>,
}

#[account]
pub struct ProfileData {
    pub owner:        Pubkey, // 本来は検証すべきフィールド
    pub burned_total: u64,
    pub cap:          u64,
}

#[account]
pub struct EarnStatus {
    pub earned:  u64,
    pub claimed: u64,
}
