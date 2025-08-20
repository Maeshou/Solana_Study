use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf835mvTWf");

#[program]
pub mod pattern_835 {
    use super::*;

    pub fn execute(ctx: Context<Ctx835>, initial: u64, info: String) -> Result<()> {
        // Double init
        state.value = initial.checked_mul(2).unwrap();
        // Metadata prefix
        state.info = format!("> {}", info);
        state.len = state.info.len() as u64;
        msg!("Case 835: executed with ops ['double_init', 'metadata']");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx835<'info> {
    #[account(init, seeds = [b"state", user.key().as_ref()], bump, payer = user, space = 8 + 1 + 32 + 256)]
    pub state: Account<'info, State835>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct State835 {
    pub bump: u8,
    pub owner: Pubkey,
    pub storage: [u8; 128],
}
