use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf399mvTWf");

#[program]
pub mod craft_coffer_399 {
    use super::*;

    pub fn initialize_coffer(ctx: Context<InitializeCoffer399>, info_str: String) -> Result<()> {
        // Store initial metadata
        ctx.accounts.record.info = info_str.clone();
        msg!("Case 399: craft coffer info '{}'", info_str);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCoffer399<'info> {
    #[account(init, seeds = [b"coffer", payer.key().as_ref()], bump, payer = payer, space = 8 + 32 + 4 + info_str.len())]
    pub record: Account<'info, Record399>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Record399 {
    pub info: String,
}
