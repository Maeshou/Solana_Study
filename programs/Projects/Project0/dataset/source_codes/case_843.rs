use anchor_lang::prelude::*;
use anchor_spl::associated_token as ata;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf843mvTWf");

#[program]
pub mod pattern_843 {
    use super::*;

    pub fn execute(ctx: Context<Ctx843>, initial: u64, info: String) -> Result<()> {
        // Double init
        state.value = initial.checked_mul(2).unwrap();
        // Metadata prefix
        state.info = format!("> {}", info);
        state.len = state.info.len() as u64;
        // ATA create
        ata::create(ctx.accounts.into());
        msg!("Case 843: executed with ops ['double_init', 'metadata', 'ata']");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx843<'info> {
    #[account(init, seeds = [b"state", user.key().as_ref()], bump, payer = user, space = 8 + 1 + 32 + 256)]
    pub state: Account<'info, State843>,
    #[account(mut)] pub user: Signer<'info>,
    #[account(address = anchor_spl::associated_token::ID)] pub ata_program: Program<'info, anchor_spl::token::Token>,
    #[account(init, associated_token::mint = mint, associated_token::authority = user)] pub ata: Account<'info, anchor_spl::token::TokenAccount>,
    pub mint: Account<'info, anchor_spl::token::Mint>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct State843 {
    pub bump: u8,
    pub owner: Pubkey,
    pub storage: [u8; 128],
}
