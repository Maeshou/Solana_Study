use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxEARNDISTINVULN000000");

#[program]
pub mod rental_earning_distribution_insecure {
    use super::*;

    /// 借り手が獲得したトークンを、事前設定された割合で
    /// 貸し主と借り手に分配し、累積を更新します。
    ///
    /// - `earned`: 借り手がこのセッションで稼いだトークン量  
    /// 署名チェックは一切行われません。
    pub fn distribute_rewards(
        ctx: Context<DistributeRewards>,
        earned: u64,
    ) {
        let data = &mut ctx.accounts.rental_data;

        // borrower_share_bps は 0–10_000 の範囲
        let bps = data.borrower_share_bps as u64;

        // 借り手取り分 = earned × borrower_share_bps / 10_000
        let borrower_amt = earned
            .checked_mul(bps).unwrap()
            .checked_div(10_000).unwrap();

        // 貸し主取り分 = earned − borrower_amt
        let lender_amt = earned.checked_sub(borrower_amt).unwrap();

        // 累積更新
        data.cumulative_borrower = data.cumulative_borrower
            .checked_add(borrower_amt).unwrap();
        data.cumulative_lender   = data.cumulative_lender
            .checked_add(lender_amt).unwrap();
    }
}

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    /// 貸し主アカウント（署名チェック omitted intentionally）
    pub owner:        AccountInfo<'info>,

    /// 借り手アカウント（署名チェック omitted intentionally）
    pub renter:       AccountInfo<'info>,

    /// 事前に init された RentalData PDA
    #[account(
        mut,
        seeds = [b"rent", owner.key().as_ref(), renter.key().as_ref()],
        bump
    )]
    pub rental_data: Account<'info, RentalData>,
}

#[account]
pub struct RentalData {
    /// NFT の貸し主
    pub owner:               Pubkey,
    /// NFT の借り手
    pub renter:              Pubkey,
    /// 借り手に渡す割合 (basis points, 10000 = 100%)
    pub borrower_share_bps:  u16,
    /// 累積借り手獲得トークン
    pub cumulative_borrower: u64,
    /// 累積貸し主獲得トークン
    pub cumulative_lender:  u64,
}
