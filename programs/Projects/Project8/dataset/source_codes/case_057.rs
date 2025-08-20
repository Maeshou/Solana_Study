
use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

declare_id!("BUMPSEED057716ID");

#[program]
pub mod bump_seed_case_057 {
    use super::*;

    pub fn launch_057(ctx: Context<FormCtx057>, bump: u8) -> Result<()> {
        let mut acc = &mut ctx.accounts.account_057;
        let start = acc.count;
        acc.count = start.checked_add(bump as u64).unwrap_or(start);
        let current = Clock::get()?.unix_timestamp as u64;
        acc.count = acc.count.wrapping_mul(current % (bump as u64 + 1));
        acc.count = acc.count.saturating_sub(bump as u64);
        msg!("case 057: final={}", acc.count);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct FormCtx057<'info> {
    #[account(init, payer = user, seeds = [b"bump_seed_case_057", user.key().as_ref(), bump.to_le_bytes().as_ref()], bump)]
    pub account_057: Account<'info, DataAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub owner: Pubkey,
    pub count: u64,
}
