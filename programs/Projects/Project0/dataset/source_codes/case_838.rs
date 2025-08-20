use anchor_lang::prelude::*;
use anchor_lang::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf838mvTWf");

#[program]
pub mod pattern_838 {
    use super::*;

    pub fn execute(ctx: Context<Ctx838>, info: String) -> Result<()> {
        // Metadata prefix
        state.info = format!("> {}", info);
        state.len = state.info.len() as u64;
        // Clock timestamp
        let clk = Clock::get()?;
        state.ts = clk.unix_timestamp as u64;
        state.slot = clk.slot;
        msg!("Case 838: executed with ops ['metadata', 'clock']");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx838<'info> {
    #[account(init, seeds = [b"state", user.key().as_ref()], bump, payer = user, space = 8 + 1 + 32 + 256)]
    pub state: Account<'info, State838>,
    #[account(mut)] pub user: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct State838 {
    pub bump: u8,
    pub owner: Pubkey,
    pub storage: [u8; 128],
}
