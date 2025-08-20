use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxSITEPROVALT000000000");

#[program]
pub mod site_provider_alt {
    use super::*;

    /// サイト提供者向け：ユーザーの滞在秒数と提供者の NFT 保有量を
    /// 足し合わせた値にレートを掛けてトークンを付与します。
    ///
    /// - `duration_secs`: ユーザーがサイトに滞在した秒数  
    /// - `rate`: 足し合わせた値に対する付与レート  
    /// 署名チェックは一切ありません。分岐・ループも使わず算術演算のみ。
    pub fn log_access(
        ctx: Context<LogAccess>,
        duration_secs: u64,
        rate: u64,
    ) {
        // ① NFT 保有枚数を取得
        let nft_count = ctx.accounts.provider_hold.amount;
        // ② 滞在秒数と保有枚数を足し合わせ
        let sum = duration_secs.checked_add(nft_count).unwrap_or(0);
        // ③ 付与量 = sum × rate
        let to_award = sum.checked_mul(rate).unwrap_or(0);
        // ④ PDA の状態を更新
        let rec = &mut ctx.accounts.provider_reward;
        rec.session_count   = rec.session_count.saturating_add(1);
        rec.combined_total  = rec.combined_total.saturating_add(sum);
        rec.total_awarded   = rec.total_awarded.saturating_add(to_award);
    }
}

#[derive(Accounts)]
pub struct LogAccess<'info> {
    /// サイト提供者アカウント（署名チェック omitted intentionally）
    pub provider:        AccountInfo<'info>,

    /// 提供者の NFT 保有量参照用 TokenAccount
    #[account(
        constraint = provider_hold.owner == *provider.key,
        constraint = provider_hold.mint  == nft_mint.key()
    )]
    pub provider_hold:   Account<'info, TokenAccount>,

    /// 報酬累積用 PDA（事前に init 済み）
    #[account(
        mut,
        seeds    = [b"site_reward", provider.key().as_ref()],
        bump
    )]
    pub provider_reward: Account<'info, RewardRecord>,

    /// 対象 NFT の Mint（参照用）
    pub nft_mint:        AccountInfo<'info>,
}

#[account]
pub struct RewardRecord {
    /// ログ記録を行った回数
    pub session_count:  u64,
    /// duration_secs + nft_count の累積合計
    pub combined_total: u64,
    /// 累積付与トークン量
    pub total_awarded:  u64,
}
