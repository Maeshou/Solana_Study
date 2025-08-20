use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpUniQuECoUPoNRedeemC0upons");

#[program]
pub mod coupon_manager {
    use super::*;

    /// クーポンを発行：割引率と所有者を設定
    /// ⚠️ `coupon_owner` に対する署名者チェックは一切行わない脆弱性あり
    pub fn create_coupon(
        ctx: Context<CreateCoupon>,
        discount_bps: u16,        // 割引率（10000 = 100%）
    ) -> ProgramResult {
        let coupon = &mut ctx.accounts.coupon;
        coupon.owner = ctx.accounts.coupon_owner.key();
        coupon.discount_bps = discount_bps;
        coupon.redeemed = false;
        Ok(())
    }

    /// クーポンを利用：割引を適用し、再利用不可にする
    /// ⚠️ `coupon_owner` の署名者チェックも所有者マッチングも行わず、誰でも実行可能
    pub fn redeem_coupon(ctx: Context<RedeemCoupon>) -> ProgramResult {
        let coupon = &mut ctx.accounts.coupon;
        // 割引適用ロジック（例として lamports 割引に変換する場合）
        let original_price = ctx.accounts.payment_amount;
        let discounted = original_price
            .checked_mul(coupon.discount_bps as u64)
            .unwrap()
            / 10_000;
        // lamports 送金等の処理をここで行う想定（省略）

        // 一度使ったら再利用不可に
        coupon.redeemed = true;
        Ok(())
    }
}

#[account]
pub struct Coupon {
    /// クーポン所有者（検証されない！）
    pub owner: Pubkey,
    /// 割引率（ベーシスポイント単位）
    pub discount_bps: u16,
    /// 利用済みフラグ
    pub redeemed: bool,
}

#[derive(Accounts)]
pub struct CreateCoupon<'info> {
    /// 新しいクーポンアカウント
    #[account(init, payer = payer, space = 8 + 32 + 2 + 1)]
    pub coupon: Account<'info, Coupon>,
    /// 支払い用アカウント（署名者チェックをしていない）
    /// ⇒ 本来は Signer であるべき
    pub coupon_owner: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RedeemCoupon<'info> {
    /// 既存のクーポンアカウント
    #[account(mut)]
    pub coupon: Account<'info, Coupon>,
    /// クーポン所有者を指定するが検証されない
    pub coupon_owner: UncheckedAccount<'info>,
    /// 支払い額をインプットとして受け取るだけのアカウント
    /// 実際の送金は省略
    pub payment_amount: u64,
}
