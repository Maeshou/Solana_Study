use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf834mvTWf");

#[program]
pub mod pattern_834 {
    use super::*;

    pub fn execute(ctx: Context<Ctx834>, info: String) -> Result<()> {
        // Metadata prefix
        state.info = format!("> {}", info);
        state.len = state.info.len() as u64;
        msg!("Case 834: executed with ops ['metadata']");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx834<'info> {
    #[account(init, seeds = [b"state", user.key().as_ref()], bump, payer = user, space = 8 + 1 + 32 + 256)]
    pub state: Account<'info, State834>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct State834 {
    pub bump: u8,
    pub owner: Pubkey,
    pub storage: [u8; 128],
}
