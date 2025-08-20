use anchor_lang::prelude::*;

declare_id!("BidBoard8888888888888888888888888888888888");

#[program]
pub mod bid_board {
    use super::*;

    pub fn submit(ctx: Context<SubmitBid>, amount: u64) -> Result<()> {
        let b = &mut ctx.accounts.board;
        b.bids.push((ctx.accounts.user.key(), amount));
        b.bids.sort_by(|a, b| b.1.cmp(&a.1));
        if b.bids.len() > b.capacity as usize {
            b.bids.pop();
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitBid<'info> {
    #[account(mut)]
    pub board: Account<'info, BoardData>,
    pub user: Signer<'info>,
}

#[account]
pub struct BoardData {
    pub bids: Vec<(Pubkey, u64)>,
    pub capacity: u8,
}
