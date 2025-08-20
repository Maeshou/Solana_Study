use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf374mvTWf");

#[program]
pub mod launch_store_374 {
    use super::*;

    pub fn initialize_store(ctx: Context<InitializeStore374>, info_str: String) -> Result<()> {
        // Store initial metadata
        ctx.accounts.record.info = info_str.clone();
        msg!("Case 374: launch store info '{}'", info_str);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeStore374<'info> {
    #[account(init, seeds = [b"store", payer.key().as_ref()], bump, payer = payer, space = 8 + 32 + 4 + info_str.len())]
    pub record: Account<'info, Record374>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Record374 {
    pub info: String,
}
