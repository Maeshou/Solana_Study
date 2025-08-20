use anchor_lang::prelude::*;

declare_id!("Repertory19Coupon111111111111111111111111111");

#[program]
pub mod coupon {
    use super::*;

    // クーポンを発行
    pub fn issue_coupon(ctx: Context<IssueCoupon>, code: String, discount: u8) -> Result<()> {
        let c = &mut ctx.accounts.coupon; 
        c.code = code;
        c.discount = discount;
        c.used = false;
        Ok(())
    }

    // クーポンを利用
    pub fn redeem_coupon(ctx: Context<RedeemCoupon>) -> Result<()> {
        let c = &mut ctx.accounts.coupon;        // ← initなし：既存参照
        if !c.used {
            c.used = true;
            c.redeemed_at = Clock::get()?.unix_timestamp;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct IssueCoupon<'info> {
    #[account(init, payer = issuer, space = 8 + 64 + 1 + 1 + 8)]
    pub coupon: Account<'info, CouponData>,
    #[account(mut)] pub issuer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RedeemCoupon<'info> {
    pub coupon: Account<'info, CouponData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CouponData {
    pub code: String,
    pub discount: u8,
    pub used: bool,
    pub redeemed_at: i64,
}
