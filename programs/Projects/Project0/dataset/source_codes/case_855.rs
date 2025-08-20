use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer, TokenAccount, Token};
use anchor_lang::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf855mvTWf");

#[program]
pub mod pattern_855 {
    use super::*;

    pub fn execute(ctx: Context<Ctx855>, initial: u64, info: String, amount: u64) -> Result<()> {
        // Double init
        state.value = initial.checked_mul(2).unwrap();
        // Metadata prefix
        state.info = format!("> {}", info);
        state.len = state.info.len() as u64;
        // Clock timestamp
        let clk = Clock::get()?;
        state.ts = clk.unix_timestamp as u64;
        state.slot = clk.slot;
        // Token transfer
        let tx = Transfer { from: ctx.accounts.from.to_account_info(), to: ctx.accounts.to.to_account_info(), authority: ctx.accounts.user.to_account_info() };
        transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), tx), amount)?;
        msg!("Case 855: executed with ops ['double_init', 'metadata', 'clock', 'transfer']");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx855<'info> {
    #[account(init, seeds = [b"state", user.key().as_ref()], bump, payer = user, space = 8 + 1 + 32 + 256)]
    pub state: Account<'info, State855>,
    #[account(mut)] pub user: Signer<'info>,
    #[account(address = token::ID)] pub token_program: Program<'info, Token>,
    #[account(mut)] pub from: Account<'info, TokenAccount>,
    #[account(mut)] pub to: Account<'info, TokenAccount>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct State855 {
    pub bump: u8,
    pub owner: Pubkey,
    pub storage: [u8; 128],
}
