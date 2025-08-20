use anchor_lang::prelude::*;

declare_id!("VulnAuc6666666666666666666666666666666666");

#[program]
pub mod vuln_auction {
    pub fn bid(ctx: Context<Bid>, amount: u64) -> Result<()> {
        // auction.owner 検証していない
        let auc = &mut ctx.accounts.auction;
        auc.highest = auc.highest.max(amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Bid<'info> {
    #[account(mut)]
    pub auction: Account<'info, AuctionData>,
    pub bidder: Signer<'info>,
}

#[account]
pub struct AuctionData {
    pub owner: Pubkey,
    pub highest: u64,
}
