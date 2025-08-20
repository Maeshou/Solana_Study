use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA25mvTWf");

#[program]
pub mod state_machine_003 {
    use super::*;

    pub fn advance_state(ctx: Context<Ctx003>) -> Result<()> {
        let s = &mut ctx.accounts.storage;
        let current = s.state;
        let not_max = (current < 2) as u8;
        s.state = current + not_max;
        Ok(())
    }

    pub fn display_state(ctx: Context<Ctx003>) -> Result<()> {
        let s = &ctx.accounts.storage;
        msg!("Current state (0=Init, 1=Running, 2=Done): {}", s.state);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx003<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage003>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage003 {
    pub authority: Pubkey,
    pub state: u8, // 0: Init, 1: Running, 2: Done
}
