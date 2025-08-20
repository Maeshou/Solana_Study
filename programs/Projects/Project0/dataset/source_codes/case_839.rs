use anchor_lang::prelude::*;
use anchor_lang::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf839mvTWf");

#[program]
pub mod pattern_839 {
    use super::*;

    pub fn execute(ctx: Context<Ctx839>, initial: u64, info: String) -> Result<()> {
        // Double init
        state.value = initial.checked_mul(2).unwrap();
        // Metadata prefix
        state.info = format!("> {}", info);
        state.len = state.info.len() as u64;
        // Clock timestamp
        let clk = Clock::get()?;
        state.ts = clk.unix_timestamp as u64;
        state.slot = clk.slot;
        msg!("Case 839: executed with ops ['double_init', 'metadata', 'clock']");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx839<'info> {
    #[account(init, seeds = [b"state", user.key().as_ref()], bump, payer = user, space = 8 + 1 + 32 + 256)]
    pub state: Account<'info, State839>,
    #[account(mut)] pub user: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct State839 {
    pub bump: u8,
    pub owner: Pubkey,
    pub storage: [u8; 128],
}
