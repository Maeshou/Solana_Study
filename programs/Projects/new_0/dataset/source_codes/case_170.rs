use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUO");

#[program]
pub mod auction_service {
    use super::*;

    /// オークション初期化：主要フィールドだけセットし、残りはDefaultで
    pub fn initialize_auction(
        ctx: Context<InitializeAuction>,
        auction_id: u64,
        item: String,
        duration_secs: i64,
    ) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        let now = ctx.accounts.clock.unix_timestamp;

        *auction = Auction {
            owner:          ctx.accounts.seller.key(),
            bump:           *ctx.bumps.get("auction").unwrap(),
            auction_id,
            item,
            end_ts:         now + duration_secs,
            last_action_ts: now,
            ..Default::default()
        };
        Ok(())
    }

    /// 入札：期限切れ判定と、最高入札額更新を独立して処理
    pub fn place_bid(
        ctx: Context<PlaceBid>,
        amount: u64,
    ) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        let now = ctx.accounts.clock.unix_timestamp;

        // 期限到来なら閉鎖フラグを立てる
        if now >= auction.end_ts {
            auction.closed = true;
        }

        // クローズでなく、かつ新しい最高額なら更新
        if !auction.closed && amount > auction.highest_bid {
            auction.highest_bid    = amount;
            auction.highest_bidder = ctx.accounts.bidder.key();
        }

        // 最終操作時刻は常に更新
        auction.last_action_ts = now;
        Ok(())
    }

    /// オークション終了処理：期限到来で閉鎖
    pub fn finalize_auction(
        ctx: Context<FinalizeAuction>,
    ) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        let now = ctx.accounts.clock.unix_timestamp;

        if !auction.closed && now >= auction.end_ts {
            auction.closed         = true;
            auction.last_action_ts = now;
        }
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(auction_id: u64)]
pub struct InitializeAuction<'info> {
    /// ゼロクリア後、Defaultで初期化可能に
    #[account(
        init_zeroed,
        payer = seller,
        seeds = [b"auction", seller.key().as_ref(), &auction_id.to_le_bytes()],
        bump,
        space = 8   // discriminator
              +32  // owner
              +1   // bump
              +8   // auction_id
              +4+64// item (max 64 bytes)
              +8   // end_ts
              +8   // highest_bid
              +32  // highest_bidder
              +1   // closed
              +8   // last_action_ts
    )]
    pub auction: Account<'info, Auction>,

    /// 出品者（署名必須）
    #[account(mut)]
    pub seller: Signer<'info>,

    /// 時刻取得用
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceBid<'info> {
    #[account(
        mut,
        seeds = [b"auction", auction.owner.as_ref(), &auction.auction_id.to_le_bytes()],
        bump = auction.bump,
    )]
    pub auction: Account<'info, Auction>,

    /// 入札者（署名必須）
    #[account(signer)]
    pub bidder: AccountInfo<'info>,

    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FinalizeAuction<'info> {
    #[account(
        mut,
        seeds = [b"auction", owner.key().as_ref(), &auction.auction_id.to_le_bytes()],
        bump = auction.bump,
        has_one = owner
    )]
    pub auction: Account<'info, Auction>,

    /// 出品者（署名必須）
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct Auction {
    pub owner:           Pubkey,  // 出品者
    pub bump:            u8,      // PDA bump
    pub auction_id:      u64,     // オークションID
    pub item:            String,  // 出品アイテム名
    pub end_ts:          i64,     // 終了タイムスタンプ
    pub highest_bid:     u64,     // 最高入札額
    pub highest_bidder:  Pubkey,  // 最高入札者
    pub closed:          bool,    // 終了フラグ
    pub last_action_ts:  i64,     // 最終操作タイムスタンプ
}
