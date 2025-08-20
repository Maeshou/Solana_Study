use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer, TokenAccount, Token};
use anchor_lang::sysvar::clock::Clock;
use anchor_lang::sysvar::rent::Rent;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf989mvTWf");

#[program]
pub mod pattern_989 {
    use super::*;

    pub fn execute(ctx: Context<Ctx989>) -> Result<()> {
        // Record timestamp
        let clk = Clock::get()?;
        ctx.accounts.state.last_ts = clk.unix_timestamp as u64;
        // Record rent-exempt balance
        let rent_bal = ctx.accounts.rent.minimum_balance(0);
        ctx.accounts.state.rent_exempt = rent_bal;
        // Store metadata byte
        let b = ctx.accounts.pool.destination.key().to_bytes()[0];
        ctx.accounts.state.meta_byte = b;
        msg!("Case 989: executed ops ['timestamp', 'rent', 'meta_byte']");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx989<'info> {
    #[account(has_one = vault, has_one = destination, seeds = [destination.key().as_ref()], bump = pool.bump)]
    pub pool: Account<'info, Pool989>,
    #[account(mut)] pub vault: Account<'info, TokenAccount>,
    #[account(mut)] pub destination: Account<'info, TokenAccount>,
    #[account(init, seeds = [b"state", pool.to_account_info().key.as_ref()], bump, payer = user, space = 8 + 1 + 32 + 8 + 8 + 8 + 32 + 1)]
    pub state: Account<'info, State989>,
    #[account(signer)] pub user: Signer<'info>,
    #[account(address = token::ID)] pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Pool989 {
    pub vault: Pubkey,
    pub destination: Pubkey,
    pub bump: u8,
}

#[account]
pub struct State989 {
    pub bump: u8,
    pub counter: u64,
    pub last_amount: u64,
    pub last_ts: u64,
    pub rent_exempt: u64,
    pub user: Pubkey,
    pub meta_byte: u8,
}