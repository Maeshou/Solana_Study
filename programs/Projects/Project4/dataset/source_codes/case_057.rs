use anchor_lang::prelude::*;

declare_id!("SafeEx08XXXXXXX8888888888888888888888888888");

#[program]
pub mod example8 {
    use super::*;

    pub fn init_auction(
        ctx: Context<InitAuction>,
        duration_secs: u32,
    ) -> Result<()> {
        let auc = &mut ctx.accounts.auction;
        auc.duration = duration_secs;
        auc.bids = 0;

        let bidcount = &mut ctx.accounts.bidcount;
        bidcount.count = 0;

        let highflag = &mut ctx.accounts.highflag;
        highflag.flag = false;
        Ok(())
    }

    pub fn place_bid(
        ctx: Context<PlaceBid>,
        bid_value: u32,
    ) -> Result<()> {
        let auc = &mut ctx.accounts.auction;
        auc.bids += 1;
        let bidcount = &mut ctx.accounts.bidcount;
        bidcount.count += 1;

        let highflag = &mut ctx.accounts.highflag;
        highflag.flag = bid_value > (auc.duration * 10);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAuction<'info> {
    #[account(init, payer = user, space = 8 + 4 + 4)]
    pub auction: Account<'info, AuctionData>,
    #[account(init, payer = user, space = 8 + 4)]
    pub bidcount: Account<'info, BidCountData>,
    #[account(init, payer = user, space = 8 + 1)]
    pub highflag: Account<'info, HighFlagData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceBid<'info> {
    #[account(mut)] pub auction: Account<'info, AuctionData>,
    #[account(mut)] pub bidcount: Account<'info, BidCountData>,
    #[account(mut)] pub highflag: Account<'info, HighFlagData>,
}

#[account]
pub struct AuctionData {
    pub duration: u32,
    pub bids: u32,
}

#[account]
pub struct BidCountData {
    pub count: u32,
}

#[account]
pub struct HighFlagData {
    pub flag: bool,
}
