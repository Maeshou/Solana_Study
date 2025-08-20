use anchor_lang::prelude::*;
declare_id!("Coup0n11111111111111111111111111111111111");

/// クーポン情報
#[account]
pub struct Coupon {
    pub issuer:      Pubkey, // クーポン発行者
    pub code:        String, // クーポンコード
    pub total_uses:  u64,    // 残り利用回数
}

/// クーポン利用記録
#[account]
pub struct CouponUsage {
    pub user:   Pubkey, // 利用者
    pub coupon: Pubkey, // 本来はここが Coupon.key() と一致すべき
    pub used:   bool,   // 利用済みフラグ
}

#[derive(Accounts)]
pub struct InitializeCoupon<'info> {
    #[account(init, payer = issuer, space = 8 + 32 + 4 + 32 + 8)]
    pub coupon:        Account<'info, Coupon>,
    #[account(mut)]
    pub issuer:        Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UseCoupon<'info> {
    /// Coupon.issuer == issuer.key() はチェックされる
    #[account(mut, has_one = issuer)]
    pub coupon:       Account<'info, Coupon>,

    /// CouponUsage.coupon == coupon.key() の検証がないため、
    /// 攻撃者は別の CouponUsage アカウントを渡して脆弱性を突ける
    #[account(mut)]
    pub usage:        Account<'info, CouponUsage>,

    pub issuer:      Signer<'info>,
}

#[program]
pub mod coupon_vuln {
    use super::*;

    /// クーポンを初期化
    pub fn initialize_coupon(ctx: Context<InitializeCoupon>, code: String, max_uses: u64) -> Result<()> {
        let c = &mut ctx.accounts.coupon;
        c.issuer     = ctx.accounts.issuer.key();
        c.code       = code;
        c.total_uses = max_uses;
        msg!("Coupon {} issued by {}", c.code, c.issuer);
        Ok(())
    }

    /// クーポン利用
    pub fn use_coupon(ctx: Context<UseCoupon>) -> Result<()> {
        let c = &mut ctx.accounts.coupon;
        let u = &mut ctx.accounts.usage;

        // 本来必要なチェック例:
        // require_keys_eq!(
        //     u.coupon,
        //     c.key(),
        //     CouponError::CouponMismatch
        // );
        //
        // または
        // #[account(address = coupon.key())] pub usage: Account<'info, CouponUsage>,

        // 脆弱性ポイント：上のどちらもないため、攻撃者は
        // 自分の使用レコードを渡して c.total_uses を好きに操作できる

        u.user   = ctx.accounts.issuer.key();
        u.used   = true;
        c.total_uses = c.total_uses.checked_sub(1).unwrap();

        msg!(
            "User {} used coupon {}, remaining uses: {}",
            u.user,
            c.code,
            c.total_uses
        );
        Ok(())
    }
}

#[error_code]
pub enum CouponError {
    #[msg("CouponUsage が指定の Coupon と一致しません")]
    CouponMismatch,
}
