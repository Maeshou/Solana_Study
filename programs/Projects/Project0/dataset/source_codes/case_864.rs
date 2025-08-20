use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf864mvTWf");

#[program]
pub mod pattern_864 {
    use super::*;

    pub fn execute(ctx: Context<Ctx864>, lamports: u64) -> Result<()> {
        // System transfer
        let tx = system_program::Transfer { from: ctx.accounts.payer.to_account_info(), to: ctx.accounts.receiver.to_account_info() };
        system_program::transfer(CpiContext::new(ctx.accounts.sys_prog.to_account_info(), tx), lamports)?;
        msg!("Case 864: executed with ops ['system']");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx864<'info> {
    #[account(init, seeds = [b"state", user.key().as_ref()], bump, payer = user, space = 8 + 1 + 32 + 256)]
    pub state: Account<'info, State864>,
    #[account(mut)] pub user: Signer<'info>,
    #[account(address = system_program::ID)] pub sys_prog: Program<'info, System>,
    #[account(mut)] pub payer: Signer<'info>,
    #[account(mut)] pub receiver: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct State864 {
    pub bump: u8,
    pub owner: Pubkey,
    pub storage: [u8; 128],
}
