use anchor_lang::prelude::*;
use anchor_spl::associated_token as ata;
use anchor_lang::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf844mvTWf");

#[program]
pub mod pattern_844 {
    use super::*;

    pub fn execute(ctx: Context<Ctx844>) -> Result<()> {
        // Clock timestamp
        let clk = Clock::get()?;
        state.ts = clk.unix_timestamp as u64;
        state.slot = clk.slot;
        // ATA create
        ata::create(ctx.accounts.into());
        msg!("Case 844: executed with ops ['clock', 'ata']");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx844<'info> {
    #[account(init, seeds = [b"state", user.key().as_ref()], bump, payer = user, space = 8 + 1 + 32 + 256)]
    pub state: Account<'info, State844>,
    #[account(mut)] pub user: Signer<'info>,
    #[account(address = anchor_spl::associated_token::ID)] pub ata_program: Program<'info, anchor_spl::token::Token>,
    #[account(init, associated_token::mint = mint, associated_token::authority = user)] pub ata: Account<'info, anchor_spl::token::TokenAccount>,
    pub mint: Account<'info, anchor_spl::token::Mint>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct State844 {
    pub bump: u8,
    pub owner: Pubkey,
    pub storage: [u8; 128],
}
