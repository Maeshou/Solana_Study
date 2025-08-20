use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf329mvTWf");

#[program]
pub mod craft_coffer_329 {
    use super::*;

    pub fn initialize_coffer(ctx: Context<InitializeCoffer329>, info_str: String) -> Result<()> {
        // Store initial metadata
        ctx.accounts.record.info = info_str.clone();
        msg!("Case 329: craft coffer info '{}'", info_str);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCoffer329<'info> {
    #[account(init, seeds = [b"coffer", payer.key().as_ref()], bump, payer = payer, space = 8 + 32 + 4 + info_str.len())]
    pub record: Account<'info, Record329>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Record329 {
    pub info: String,
}
