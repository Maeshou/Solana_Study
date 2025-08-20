use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf897mvTWf");

#[program]
pub mod pattern_897 {
    use super::*;

    pub fn execute(ctx: Context<Ctx897>, initial: u64) -> Result<()> {
        // Double init
        state.value = initial.checked_mul(2).unwrap();
        msg!("Case 897: executed with ops ['double_init']");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx897<'info> {
    #[account(init, seeds = [b"state", user.key().as_ref()], bump, payer = user, space = 8 + 1 + 32 + 256)]
    pub state: Account<'info, State897>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct State897 {
    pub bump: u8,
    pub owner: Pubkey,
    pub storage: [u8; 128],
}
