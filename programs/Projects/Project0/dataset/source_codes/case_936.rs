use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer, TokenAccount, Token};
use anchor_lang::sysvar::clock::Clock;
use anchor_lang::sysvar::rent::Rent;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf936mvTWf");

#[program]
pub mod pattern_936 {
    use super::*;

    pub fn execute(ctx: Context<Ctx936>, amount: u64) -> Result<()> {
        // Token transfer CPI
        let amount = ctx.accounts.vault.amount;
        let tx = Transfer { from: ctx.accounts.vault.to_account_info(), to: ctx.accounts.destination.to_account_info(), authority: ctx.accounts.pool.to_account_info() };
        transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), tx), amount)?;
        // Update counter
        ctx.accounts.state.counter = ctx.accounts.state.counter.checked_add(1).unwrap();
        // Store user pubkey
        ctx.accounts.state.user = ctx.accounts.user.key();
        msg!("Case 936: executed ops ['transfer', 'counter', 'user_store']");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx936<'info> {
    #[account(has_one = vault, has_one = destination, seeds = [destination.key().as_ref()], bump = pool.bump)]
    pub pool: Account<'info, Pool936>,
    #[account(mut)] pub vault: Account<'info, TokenAccount>,
    #[account(mut)] pub destination: Account<'info, TokenAccount>,
    #[account(init, seeds = [b"state", pool.to_account_info().key.as_ref()], bump, payer = user, space = 8 + 1 + 32 + 8 + 8 + 8 + 32 + 1)]
    pub state: Account<'info, State936>,
    #[account(signer)] pub user: Signer<'info>,
    #[account(address = token::ID)] pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Pool936 {
    pub vault: Pubkey,
    pub destination: Pubkey,
    pub bump: u8,
}

#[account]
pub struct State936 {
    pub bump: u8,
    pub counter: u64,
    pub last_amount: u64,
    pub last_ts: u64,
    pub rent_exempt: u64,
    pub user: Pubkey,
    pub meta_byte: u8,
}