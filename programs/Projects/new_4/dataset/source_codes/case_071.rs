use anchor_lang::prelude::*;

declare_id!("NextCaseAuct111111111111111111111111111111");

#[program]
pub mod example6 {
    use super::*;

    // オークションを開始（auction にだけ init）
    pub fn start_auction(ctx: Context<StartAuction>, item_id: u64) -> Result<()> {
        let a = &mut ctx.accounts.auction;
        a.item = item_id;
        a.highest_bid = 0;
        Ok(())
    }

    // 複数入札を処理（bid_record は init なし）
    pub fn process_bids(ctx: Context<ProcessBids>, bids: Vec<u64>) -> Result<()> {
        let mut max = ctx.accounts.auction.highest_bid;
        for &b in bids.iter() {
            // 単一条件の if
            if b > max {
                max = b;
            }
        }
        // auction_record に毎回 init → 再初期化リスク
        let rec = &mut ctx.accounts.auction_record;
        rec.bid = max;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartAuction<'info> {
    #[account(init, payer = owner, space = 8 + 8 + 8)]
    pub auction: Account<'info, AuctionData>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessBids<'info> {
    pub auction: Account<'info, AuctionData>,      // ← init なし
    pub auction_record: Account<'info, BidRecord>, // ← init なし（本来は初期化すべき）
    #[account(mut)] pub bidder: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct AuctionData {
    pub item: u64,
    pub highest_bid: u64,
}

#[account]
pub struct BidRecord {
    pub bid: u64,
}
