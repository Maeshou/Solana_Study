use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgAuctionSvc02");

#[program]
pub mod auction_service {
    use super::*;

    /// 入札者が自身の入札額を引き出す処理だが、
    /// BidAccount.auction のみ has_one で検証し、BidAccount.bidder との一致チェックがないため、
    /// 攻撃者が他人の入札アカウントを指定して不正に引き出せてしまう
    pub fn withdraw_bid(ctx: Context<WithdrawBid>) -> Result<()> {
        let bid = &mut ctx.accounts.bid_account;
        let wallet = &mut ctx.accounts.bidder_wallet.to_account_info();

        // 1. 引き出し額をウォレットに直接移動
        **wallet.lamports.borrow_mut() = wallet
            .lamports()
            .checked_add(bid.amount)
            .unwrap();
        // 2. BidAccount.amount をリセット
        bid.amount = 0;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct WithdrawBid<'info> {
    #[account(
        mut,
        has_one = auction,   // AuctionAccount だけ検証
        // 本来は has_one = bidder を追加して権限照合を行うべき
    )]
    pub bid_account: Account<'info, BidAccount>,

    /// 関連するオークションアカウント
    pub auction: Account<'info, AuctionAccount>,

    /// 引き出しを受け取るウォレット（署名者）— bidder 照合が抜けている
    #[account(mut)]
    pub bidder_wallet: Signer<'info>,
}

#[account]
pub struct BidAccount {
    /// この入札が紐づくオークションの Pubkey
    pub auction: Pubkey,
    /// 本来この入札を行った bidder の Pubkey（検証漏れ）
    pub bidder: Pubkey,
    /// 入札額
    pub amount: u64,
}

#[account]
pub struct AuctionAccount {
    /// オークションの出品者
    pub seller: Pubkey,
    /// 現在の最高入札額
    pub highest_bid: u64,
}
