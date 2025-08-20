use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf866mvTWf");

#[program]
pub mod pattern_866 {
    use super::*;

    pub fn execute(ctx: Context<Ctx866>, info: String, lamports: u64) -> Result<()> {
        // Metadata prefix
        state.info = format!("> {}", info);
        state.len = state.info.len() as u64;
        // System transfer
        let tx = system_program::Transfer { from: ctx.accounts.payer.to_account_info(), to: ctx.accounts.receiver.to_account_info() };
        system_program::transfer(CpiContext::new(ctx.accounts.sys_prog.to_account_info(), tx), lamports)?;
        msg!("Case 866: executed with ops ['metadata', 'system']");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx866<'info> {
    #[account(init, seeds = [b"state", user.key().as_ref()], bump, payer = user, space = 8 + 1 + 32 + 256)]
    pub state: Account<'info, State866>,
    #[account(mut)] pub user: Signer<'info>,
    #[account(address = system_program::ID)] pub sys_prog: Program<'info, System>,
    #[account(mut)] pub payer: Signer<'info>,
    #[account(mut)] pub receiver: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct State866 {
    pub bump: u8,
    pub owner: Pubkey,
    pub storage: [u8; 128],
}
