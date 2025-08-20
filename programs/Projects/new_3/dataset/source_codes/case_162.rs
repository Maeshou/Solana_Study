use anchor_lang::prelude::*;
declare_id!("CoupVulnNoCheckedAdd111111111111111111111");

/// デジタルクーポン情報
#[account]
pub struct Coupon {
    pub issuer:    Pubkey, // 発行者
    pub code:      String, // コード
    pub remaining: u64,    // 残り使用可能回数
}

/// クーポン利用記録
#[account]
pub struct CouponUsage {
    pub user:       Pubkey, // 利用者
    pub coupon:     Pubkey, // 本来は Coupon.key() と一致すべき
    pub times_used: u64,    // 使用回数
}

#[derive(Accounts)]
pub struct InitializeCoupon<'info> {
    #[account(init, payer = issuer, space = 8 + 32 + 4 + 32 + 8)]
    pub coupon:         Account<'info, Coupon>,
    #[account(mut)]
    pub issuer:         Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UseCoupon<'info> {
    /// Coupon.issuer == issuer.key() は検証される
    #[account(mut, has_one = issuer)]
    pub coupon:         Account<'info, Coupon>,

    /// CouponUsage.coupon ⇔ Coupon.key() の検証がないため、
    /// 任意の Usage アカウントを渡して残数操作が可能
    #[account(mut)]
    pub usage:          Account<'info, CouponUsage>,

    pub issuer:         Signer<'info>,
}

#[program]
pub mod coupon_vuln_no_checked_add {
    use super::*;

    /// クーポンを初期化
    pub fn initialize_coupon(ctx: Context<InitializeCoupon>, code: String, max_uses: u64) -> Result<()> {
        let c = &mut ctx.accounts.coupon;
        c.issuer    = ctx.accounts.issuer.key();
        c.code      = code;
        c.remaining = max_uses;
        Ok(())
    }

    /// クーポンを利用
    pub fn use_coupon(ctx: Context<UseCoupon>) -> Result<()> {
        let c = &mut ctx.accounts.coupon;
        let u = &mut ctx.accounts.usage;

        // 本来は必須：require_keys_eq!(u.coupon, c.key(), ErrorCode::CouponMismatch);

        // 検証がないため任意の CouponUsage を渡せるが
        u.user       = ctx.accounts.issuer.key();
        u.coupon     = c.key();

        // .checked_add(1).unwrap() を避け、saturating_add を使用
        u.times_used = u.times_used.saturating_add(1);
        // 残数の操作も saturating_sub で安全に実行
        c.remaining  = c.remaining.saturating_sub(1);
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("CouponUsage が指定の Coupon と一致しません")]
    CouponMismatch,
}
