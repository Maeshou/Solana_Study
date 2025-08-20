use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_lang::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf869mvTWf");

#[program]
pub mod pattern_869 {
    use super::*;

    pub fn execute(ctx: Context<Ctx869>, initial: u64, lamports: u64) -> Result<()> {
        // Double init
        state.value = initial.checked_mul(2).unwrap();
        // Clock timestamp
        let clk = Clock::get()?;
        state.ts = clk.unix_timestamp as u64;
        state.slot = clk.slot;
        // System transfer
        let tx = system_program::Transfer { from: ctx.accounts.payer.to_account_info(), to: ctx.accounts.receiver.to_account_info() };
        system_program::transfer(CpiContext::new(ctx.accounts.sys_prog.to_account_info(), tx), lamports)?;
        msg!("Case 869: executed with ops ['double_init', 'clock', 'system']");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx869<'info> {
    #[account(init, seeds = [b"state", user.key().as_ref()], bump, payer = user, space = 8 + 1 + 32 + 256)]
    pub state: Account<'info, State869>,
    #[account(mut)] pub user: Signer<'info>,
    #[account(address = system_program::ID)] pub sys_prog: Program<'info, System>,
    #[account(mut)] pub payer: Signer<'info>,
    #[account(mut)] pub receiver: SystemAccount<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct State869 {
    pub bump: u8,
    pub owner: Pubkey,
    pub storage: [u8; 128],
}
