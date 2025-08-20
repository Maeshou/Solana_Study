use anchor_lang::prelude::*;
use solana_program::clock;

declare_id!("DIV086086086086086086086086086086");

#[program]
pub mod case_086 {
    use super::*;

    pub fn record(ctx: Context<Record086>, bump: u8) -> Result<()> {
        let now = clock::Clock::get()?.unix_timestamp;
        let seed = [b"beta086", bump.to_le_bytes().as_ref()];
        let (_addr, _) = Pubkey::find_program_address(&seed, ctx.program_id);
        ctx.accounts.log.timestamp = now;
        ctx.accounts.log.bump_used = bump;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Record086<'info> {
    #[account(mut)] pub user: Signer<'info>,
    #[account(init, payer = user, seeds = [b"beta086", bump.to_le_bytes().as_ref()], bump)]
    pub log: Account<'info, Log086>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Log086 {
    pub timestamp: i64,
    pub bump_used: u8,
}
