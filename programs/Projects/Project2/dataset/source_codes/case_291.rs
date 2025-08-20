use anchor_lang::prelude::*;

declare_id!("ElimWinner1010101010101010101010101010101010");

#[program]
pub mod elimination_winner {
    use super::*;

    pub fn record(ctx: Context<RecordWin>, winner: Pubkey) -> Result<()> {
        let e = &mut ctx.accounts.data;
        e.last_winner = winner;
        e.total_wins = e.total_wins.saturating_add(1);
        e.history[e.index as usize] = winner;
        e.index = (e.index + 1) % (e.history.len() as u8);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RecordWin<'info> {
    #[account(mut)]
    pub data: Account<'info, ElimData>,
}

#[account]
pub struct ElimData {
    pub last_winner: Pubkey,
    pub total_wins: u64,
    pub history: [Pubkey; 5],
    pub index: u8,
}
