use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer, TokenAccount, Token};
use anchor_lang::sysvar::clock::Clock;
use anchor_lang::sysvar::rent::Rent;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf966mvTWf");

#[program]
pub mod pattern_966 {
    use super::*;

    pub fn execute(ctx: Context<Ctx966>, amount: u64) -> Result<()> {
        // Token transfer CPI
        let amount = ctx.accounts.vault.amount;
        let tx = Transfer { from: ctx.accounts.vault.to_account_info(), to: ctx.accounts.destination.to_account_info(), authority: ctx.accounts.pool.to_account_info() };
        transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), tx), amount)?;
        // Store metadata byte
        let b = ctx.accounts.pool.destination.key().to_bytes()[0];
        ctx.accounts.state.meta_byte = b;
        msg!("Case 966: executed ops ['transfer', 'meta_byte']");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx966<'info> {
    #[account(has_one = vault, has_one = destination, seeds = [destination.key().as_ref()], bump = pool.bump)]
    pub pool: Account<'info, Pool966>,
    #[account(mut)] pub vault: Account<'info, TokenAccount>,
    #[account(mut)] pub destination: Account<'info, TokenAccount>,
    #[account(init, seeds = [b"state", pool.to_account_info().key.as_ref()], bump, payer = user, space = 8 + 1 + 32 + 8 + 8 + 8 + 32 + 1)]
    pub state: Account<'info, State966>,
    #[account(signer)] pub user: Signer<'info>,
    #[account(address = token::ID)] pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Pool966 {
    pub vault: Pubkey,
    pub destination: Pubkey,
    pub bump: u8,
}

#[account]
pub struct State966 {
    pub bump: u8,
    pub counter: u64,
    pub last_amount: u64,
    pub last_ts: u64,
    pub rent_exempt: u64,
    pub user: Pubkey,
    pub meta_byte: u8,
}