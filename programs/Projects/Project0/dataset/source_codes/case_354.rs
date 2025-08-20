use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf354mvTWf");

#[program]
pub mod launch_store_354 {
    use super::*;

    pub fn initialize_store(ctx: Context<InitializeStore354>, info_str: String) -> Result<()> {
        // Store initial metadata
        ctx.accounts.record.info = info_str.clone();
        msg!("Case 354: launch store info '{}'", info_str);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeStore354<'info> {
    #[account(init, seeds = [b"store", payer.key().as_ref()], bump, payer = payer, space = 8 + 32 + 4 + info_str.len())]
    pub record: Account<'info, Record354>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Record354 {
    pub info: String,
}
