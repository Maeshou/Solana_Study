use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf339mvTWf");

#[program]
pub mod craft_coffer_339 {
    use super::*;

    pub fn initialize_coffer(ctx: Context<InitializeCoffer339>, info_str: String) -> Result<()> {
        // Store initial metadata
        ctx.accounts.record.info = info_str.clone();
        msg!("Case 339: craft coffer info '{}'", info_str);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCoffer339<'info> {
    #[account(init, seeds = [b"coffer", payer.key().as_ref()], bump, payer = payer, space = 8 + 32 + 4 + info_str.len())]
    pub record: Account<'info, Record339>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Record339 {
    pub info: String,
}
