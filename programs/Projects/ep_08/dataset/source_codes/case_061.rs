use anchor_lang::prelude::*;
use solana_program::clock;

declare_id!("DIV061061061061061061061061061061");

#[program]
pub mod case_061 {
    use super::*;

    pub fn record(ctx: Context<Record061>, bump: u8) -> Result<()> {
        let now = clock::Clock::get()?.unix_timestamp;
        let seed = [b"beta061", bump.to_le_bytes().as_ref()];
        let (_addr, _) = Pubkey::find_program_address(&seed, ctx.program_id);
        ctx.accounts.log.timestamp = now;
        ctx.accounts.log.bump_used = bump;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Record061<'info> {
    #[account(mut)] pub user: Signer<'info>,
    #[account(init, payer = user, seeds = [b"beta061", bump.to_le_bytes().as_ref()], bump)]
    pub log: Account<'info, Log061>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Log061 {
    pub timestamp: i64,
    pub bump_used: u8,
}
