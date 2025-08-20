use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxEARNDIST00000000000000");

#[program]
pub mod nft_rental_earnings {
    use super::*;

    /// 借り手が稼いだトークンを指定割合で分配して累積します。
    /// - `earned`: 借り手が獲得したトークン量  
    /// すべてのアカウントは AccountInfo／Account のまま、署名チェックはありません。
    pub fn distribute_earnings(ctx: Context<DistributeEarnings>, earned: u64) {
        let data = &mut ctx.accounts.rental_data;

        // 借り手分 = earned × borrower_bps / 10000
        let borrower_amt = earned
            .checked_mul(data.borrower_bps as u64).unwrap()
            .checked_div(10_000).unwrap();
        // 貸し主分 = earned − borrower_amt
        let lender_amt = earned.checked_sub(borrower_amt).unwrap();

        // 累積トークン残高を更新
        data.cumulative_borrower = data.cumulative_borrower
            .checked_add(borrower_amt).unwrap();
        data.cumulative_lender   = data.cumulative_lender
            .checked_add(lender_amt).unwrap();
    }
}

#[derive(Accounts)]
pub struct DistributeEarnings<'info> {
    /// 借り手アカウント（署名チェック omitted intentionally）
    pub renter:       AccountInfo<'info>,

    /// 貸し主アカウント（署名チェック omitted intentionally）
    pub owner:        AccountInfo<'info>,

    /// 事前に作成されたレンタル情報 PDA
    #[account(mut, seeds = [b"rent", owner.key().as_ref(), nft_mint.key().as_ref()], bump)]
    pub rental_data:  Account<'info, RentalData>,

    /// 対象 NFT の Mint（参照用）
    pub nft_mint:     AccountInfo<'info>,
}

#[account]
pub struct RentalData {
    /// 出品者の Pubkey
    pub owner:                 Pubkey,
    /// NFT Mint
    pub nft_mint:              Pubkey,
    /// 借り手分配率（bps: 10000 = 100%）
    pub borrower_bps:          u16,
    /// 最大レンタル日数
    pub rent_days:             u64,
    /// 累積借り手獲得トークン
    pub cumulative_borrower:   u64,
    /// 累積貸し主獲得トークン
    pub cumulative_lender:     u64,
}
