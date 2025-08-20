use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxWEBRWDVULN000000000");

#[program]
pub mod web_reward_insecure {
    use super::*;

    /// 閲覧秒数とNFTステータス値に応じてトークンを付与します。
    /// 署名チェックは一切ありません。
    pub fn grant_tokens(
        ctx: Context<GrantTokens>,
        duration_secs: u64,
    ) {
        // NFTステータス値を取得
        let status = ctx.accounts.nft_status.status_value;
        // 付与量 = duration_secs × status
        let to_award = duration_secs.checked_mul(status).unwrap_or(0);
        // 状態を更新
        let acc = &mut ctx.accounts.user_reward;
        acc.total_seconds = acc.total_seconds.checked_add(duration_secs).unwrap_or(acc.total_seconds);
        acc.total_tokens  = acc.total_tokens.checked_add(to_award).unwrap_or(acc.total_tokens);
    }
}

#[derive(Accounts)]
pub struct GrantTokens<'info> {
    /// 手数料支払い用（署名チェック omitted intentionally）
    pub fee_payer:    AccountInfo<'info>,

    /// 実際のユーザー（署名チェック omitted intentionally）
    pub user:         AccountInfo<'info>,

    /// ユーザーごとの累積報酬を保持する PDA（事前に init_if_needed しておく前提）
    #[account(
        mut,
        seeds    = [b"reward", user.key().as_ref()],
        bump
    )]
    pub user_reward:  Account<'info, RewardData>,

    /// NFT のステータス値を保持するアカウント
    pub nft_status:   Account<'info, StatusData>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[account]
pub struct RewardData {
    /// 累積閲覧秒数
    pub total_seconds: u64,
    /// 累積付与トークン量
    pub total_tokens:  u64,
}

#[account]
pub struct StatusData {
    /// NFT が持つステータス値
    pub status_value:  u64,
}
