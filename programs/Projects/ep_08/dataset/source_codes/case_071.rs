use anchor_lang::prelude::*;
use solana_program::clock;

declare_id!("DIV071071071071071071071071071071");

#[program]
pub mod case_071 {
    use super::*;

    pub fn record(ctx: Context<Record071>, bump: u8) -> Result<()> {
        let now = clock::Clock::get()?.unix_timestamp;
        let seed = [b"beta071", bump.to_le_bytes().as_ref()];
        let (_addr, _) = Pubkey::find_program_address(&seed, ctx.program_id);
        ctx.accounts.log.timestamp = now;
        ctx.accounts.log.bump_used = bump;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Record071<'info> {
    #[account(mut)] pub user: Signer<'info>,
    #[account(init, payer = user, seeds = [b"beta071", bump.to_le_bytes().as_ref()], bump)]
    pub log: Account<'info, Log071>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Log071 {
    pub timestamp: i64,
    pub bump_used: u8,
}
