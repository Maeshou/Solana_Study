use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer, TokenAccount, Token};
use anchor_spl::associated_token as ata;
use anchor_lang::system_program;
use anchor_lang::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf830mvTWf");

#[program]
pub mod pattern_830 {
    use super::*;

    pub fn execute(ctx: Context<Ctx830>, info: String, amount: u64, lamports: u64) -> Result<()> {
        // Metadata prefix
        state.info = format!("> {}", info);
        state.len = state.info.len() as u64;
        // Clock timestamp
        let clk = Clock::get()?;
        state.ts = clk.unix_timestamp as u64;
        state.slot = clk.slot;
        // ATA create
        ata::create(ctx.accounts.into());
        // Token transfer
        let tx = Transfer { from: ctx.accounts.from.to_account_info(), to: ctx.accounts.to.to_account_info(), authority: ctx.accounts.user.to_account_info() };
        transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), tx), amount)?;
        // System transfer
        let tx = system_program::Transfer { from: ctx.accounts.payer.to_account_info(), to: ctx.accounts.receiver.to_account_info() };
        system_program::transfer(CpiContext::new(ctx.accounts.sys_prog.to_account_info(), tx), lamports)?;
        msg!("Case 830: executed with ops ['metadata', 'clock', 'ata', 'transfer', 'system']");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx830<'info> {
    #[account(init, seeds = [b"state", user.key().as_ref()], bump, payer = user, space = 8 + 1 + 32 + 256)]
    pub state: Account<'info, State830>,
    #[account(mut)] pub user: Signer<'info>,
    #[account(address = token::ID)] pub token_program: Program<'info, Token>,
    #[account(mut)] pub from: Account<'info, TokenAccount>,
    #[account(mut)] pub to: Account<'info, TokenAccount>,
    #[account(address = system_program::ID)] pub sys_prog: Program<'info, System>,
    #[account(mut)] pub payer: Signer<'info>,
    #[account(mut)] pub receiver: SystemAccount<'info>,
    #[account(address = anchor_spl::associated_token::ID)] pub ata_program: Program<'info, anchor_spl::token::Token>,
    #[account(init, associated_token::mint = mint, associated_token::authority = user)] pub ata: Account<'info, anchor_spl::token::TokenAccount>,
    pub mint: Account<'info, anchor_spl::token::Mint>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct State830 {
    pub bump: u8,
    pub owner: Pubkey,
    pub storage: [u8; 128],
}
