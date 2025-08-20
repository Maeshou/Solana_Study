use anchor_lang::prelude::*;
use solana_program::clock;

declare_id!("DIV056056056056056056056056056056");

#[program]
pub mod case_056 {
    use super::*;

    pub fn record(ctx: Context<Record056>, bump: u8) -> Result<()> {
        let now = clock::Clock::get()?.unix_timestamp;
        let seed = [b"beta056", bump.to_le_bytes().as_ref()];
        let (_addr, _) = Pubkey::find_program_address(&seed, ctx.program_id);
        ctx.accounts.log.timestamp = now;
        ctx.accounts.log.bump_used = bump;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Record056<'info> {
    #[account(mut)] pub user: Signer<'info>,
    #[account(init, payer = user, seeds = [b"beta056", bump.to_le_bytes().as_ref()], bump)]
    pub log: Account<'info, Log056>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Log056 {
    pub timestamp: i64,
    pub bump_used: u8,
}
