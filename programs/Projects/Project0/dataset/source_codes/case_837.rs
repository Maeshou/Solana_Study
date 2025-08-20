use anchor_lang::prelude::*;
use anchor_lang::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf837mvTWf");

#[program]
pub mod pattern_837 {
    use super::*;

    pub fn execute(ctx: Context<Ctx837>, initial: u64) -> Result<()> {
        // Double init
        state.value = initial.checked_mul(2).unwrap();
        // Clock timestamp
        let clk = Clock::get()?;
        state.ts = clk.unix_timestamp as u64;
        state.slot = clk.slot;
        msg!("Case 837: executed with ops ['double_init', 'clock']");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx837<'info> {
    #[account(init, seeds = [b"state", user.key().as_ref()], bump, payer = user, space = 8 + 1 + 32 + 256)]
    pub state: Account<'info, State837>,
    #[account(mut)] pub user: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct State837 {
    pub bump: u8,
    pub owner: Pubkey,
    pub storage: [u8; 128],
}
