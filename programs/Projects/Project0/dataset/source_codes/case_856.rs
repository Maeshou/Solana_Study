use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer, TokenAccount, Token};
use anchor_spl::associated_token as ata;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf856mvTWf");

#[program]
pub mod pattern_856 {
    use super::*;

    pub fn execute(ctx: Context<Ctx856>, amount: u64) -> Result<()> {
        // ATA create
        ata::create(ctx.accounts.into());
        // Token transfer
        let tx = Transfer { from: ctx.accounts.from.to_account_info(), to: ctx.accounts.to.to_account_info(), authority: ctx.accounts.user.to_account_info() };
        transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), tx), amount)?;
        msg!("Case 856: executed with ops ['ata', 'transfer']");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx856<'info> {
    #[account(init, seeds = [b"state", user.key().as_ref()], bump, payer = user, space = 8 + 1 + 32 + 256)]
    pub state: Account<'info, State856>,
    #[account(mut)] pub user: Signer<'info>,
    #[account(address = token::ID)] pub token_program: Program<'info, Token>,
    #[account(mut)] pub from: Account<'info, TokenAccount>,
    #[account(mut)] pub to: Account<'info, TokenAccount>,
    #[account(address = anchor_spl::associated_token::ID)] pub ata_program: Program<'info, anchor_spl::token::Token>,
    #[account(init, associated_token::mint = mint, associated_token::authority = user)] pub ata: Account<'info, anchor_spl::token::TokenAccount>,
    pub mint: Account<'info, anchor_spl::token::Mint>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct State856 {
    pub bump: u8,
    pub owner: Pubkey,
    pub storage: [u8; 128],
}
