use anchor_lang::prelude::*;
declare_id!("AuctVulnHasOne1111111111111111111111111");

/// オークション情報
#[account]
pub struct Auction {
    pub seller:      Pubkey, // 出品者
    pub highest_bid: u64,    // 最高入札額
    pub winner:      Pubkey, // 落札者
}

/// 入札情報
#[account]
pub struct Bid {
    pub bidder:  Pubkey, // 入札者
    pub auction: Pubkey, // 本来は Auction.key() と一致すべき
    pub amount:  u64,    // 入札額
}

#[derive(Accounts)]
pub struct CreateAuction<'info> {
    #[account(init, payer = seller, space = 8 + 32 + 8 + 32)]
    pub auction:    Account<'info, Auction>,
    #[account(mut)]
    pub seller:     Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceBid<'info> {
    #[account(mut)]
    pub auction:    Account<'info, Auction>,
    #[account(init, payer = bidder, space = 8 + 32 + 32 + 8)]
    pub bid:        Account<'info, Bid>,
    #[account(mut)]
    pub bidder:     Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FinalizeAuction<'info> {
    /// Auction.seller == seller.key() は検証される
    #[account(mut, has_one = seller)]
    pub auction:    Account<'info, Auction>,

    /// Bid.auction と auction.key() の検証がない
    #[account(mut)]
    pub bid:        Account<'info, Bid>,

    pub seller:     Signer<'info>,
}

#[program]
pub mod auction_vuln_hasone {
    use super::*;

    /// オークションを開始
    pub fn create_auction(ctx: Context<CreateAuction>) -> Result<()> {
        let a = &mut ctx.accounts.auction;
        a.seller      = ctx.accounts.seller.key();
        a.highest_bid = 0;
        a.winner      = Pubkey::default();
        Ok(())
    }

    /// 入札を行う
    pub fn place_bid(ctx: Context<PlaceBid>, amount: u64) -> Result<()> {
        let a = &mut ctx.accounts.auction;
        let b = &mut ctx.accounts.bid;

        // 脆弱性ポイント：
        // b.auction = a.key() としているが、
        // Bid.auction と Auction.key() の一致を実行時に検証しない
        b.bidder  = ctx.accounts.bidder.key();
        b.auction = a.key();
        b.amount  = amount;

        // 最高入札額と落札者を更新
        if b.amount > a.highest_bid {
            a.highest_bid = b.amount;
            a.winner      = b.bidder;
        }
        Ok(())
    }

    /// オークションを確定（出品者のみ実行可能）
    pub fn finalize(ctx: Context<FinalizeAuction>) -> Result<()> {
        let a = &mut ctx.accounts.auction;
        let b = &ctx.accounts.bid;

        // 本来は必須：
        // require_keys_eq!(
        //     b.auction,
        //     a.key(),
        //     AuctionError::BidMismatch
        // );
        // がないため、攻撃者は自分の偽 Bid を渡し、
        // 任意の Auction.winner を書き換えられる

        a.winner = b.bidder;
        Ok(())
    }
}

#[error_code]
pub enum AuctionError {
    #[msg("Bid が指定の Auction と一致しません")]
    BidMismatch,
}
