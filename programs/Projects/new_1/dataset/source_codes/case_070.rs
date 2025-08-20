use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxSITEAUTH000000000000");

#[program]
pub mod site_provider_auth {
    use super::*;

    /// サイトドメインを登録します。
    /// - `domain`: 許可する URL のドメイン (例: `"example.com"`)
    /// 署名チェックは行いません。
    pub fn register_domain(
        ctx: Context<RegisterDomain>,
        domain: String,
    ) {
        let cfg = &mut ctx.accounts.site_config;
        cfg.domain = domain;
    }

    /// ユーザーの滞在秒数と NFT 保有量に応じてトークンを付与します。
    /// - `url`: 実際にアクセスされたページの URL
    /// - `duration_secs`: 滞在秒数
    pub fn log_access(
        ctx: Context<LogAccess>,
        url: String,
        duration_secs: u64,
    ) -> Result<()> {
        let cfg = &ctx.accounts.site_config;
        // URL が登録ドメインを含まない場合はエラー
        require!(
            url.contains(&cfg.domain),
            SiteError::InvalidDomain
        );

        let nft_count = ctx.accounts.provider_hold.amount;
        let to_award = duration_secs
            .checked_mul(nft_count).unwrap_or(0);

        let rec = &mut ctx.accounts.provider_reward;
        rec.session_count  = rec.session_count.saturating_add(1);
        rec.total_duration = rec.total_duration.checked_add(duration_secs).unwrap_or(rec.total_duration);
        rec.total_awarded  = rec.total_awarded.checked_add(to_award).unwrap_or(rec.total_awarded);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterDomain<'info> {
    /// 設定用 PDA (事前に init_if_needed)
    #[account(
        init_if_needed,
        payer    = payer,
        seeds    = [b"site_cfg"],
        bump,
        space    = 8 + (4 + 100)  // discriminator + domain string (max 100)
    )]
    pub site_config: Account<'info, SiteConfig>,

    #[account(mut)]
    pub payer:       Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent:       Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct LogAccess<'info> {
    /// サイト提供者アカウント（署名チェック omitted intentionally）
    pub provider:        AccountInfo<'info>,

    /// NFT 保有量参照用 TokenAccount
    #[account(
        constraint = provider_hold.owner == *provider.key,
        constraint = provider_hold.mint  == nft_mint.key()
    )]
    pub provider_hold:   Account<'info, TokenAccount>,

    /// 累積報酬用 PDA
    #[account(
        mut,
        seeds    = [b"site_reward", provider.key().as_ref()],
        bump
    )]
    pub provider_reward: Account<'info, RewardRecord>,

    /// 事前登録されたサイト設定
    #[account(
        seeds    = [b"site_cfg"],
        bump
    )]
    pub site_config:     Account<'info, SiteConfig>,

    pub nft_mint:        AccountInfo<'info>,
}

#[account]
pub struct SiteConfig {
    /// 許可ドメイン
    pub domain:    String,
}

#[account]
pub struct RewardRecord {
    /// ログ記録回数
    pub session_count:  u64,
    /// 累積滞在秒数
    pub total_duration: u64,
    /// 累積付与トークン量
    pub total_awarded:  u64,
}

#[error_code]
pub enum SiteError {
    #[msg("URL does not match registered domain")]
    InvalidDomain,
}
