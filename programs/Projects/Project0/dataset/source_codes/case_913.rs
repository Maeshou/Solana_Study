use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer, TokenAccount, Token};
use anchor_lang::sysvar::clock::Clock;
use anchor_lang::sysvar::rent::Rent;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf913mvTWf");

#[program]
pub mod pattern_913 {
    use super::*;

    pub fn execute(ctx: Context<Ctx913>) -> Result<()> {
        // Record last amount
        ctx.accounts.state.last_amount = ctx.accounts.vault.amount;
        // Record timestamp
        let clk = Clock::get()?;
        ctx.accounts.state.last_ts = clk.unix_timestamp as u64;
        msg!("Case 913: executed ops ['last_amount', 'timestamp']");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx913<'info> {
    #[account(has_one = vault, has_one = destination, seeds = [destination.key().as_ref()], bump = pool.bump)]
    pub pool: Account<'info, Pool913>,
    #[account(mut)] pub vault: Account<'info, TokenAccount>,
    #[account(mut)] pub destination: Account<'info, TokenAccount>,
    #[account(init, seeds = [b"state", pool.to_account_info().key.as_ref()], bump, payer = user, space = 8 + 1 + 32 + 8 + 8 + 8 + 32 + 1)]
    pub state: Account<'info, State913>,
    #[account(signer)] pub user: Signer<'info>,
    #[account(address = token::ID)] pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Pool913 {
    pub vault: Pubkey,
    pub destination: Pubkey,
    pub bump: u8,
}

#[account]
pub struct State913 {
    pub bump: u8,
    pub counter: u64,
    pub last_amount: u64,
    pub last_ts: u64,
    pub rent_exempt: u64,
    pub user: Pubkey,
    pub meta_byte: u8,
}