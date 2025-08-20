use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf337mvTWf");

#[program]
pub mod establish_account_337 {
    use super::*;

    pub fn initialize_account(ctx: Context<InitializeAccount337>, initial_value: u64) -> Result<()> {
        // Initialize with a starting value
        ctx.accounts.record.value = initial_value;
        ctx.accounts.record.creator = ctx.accounts.creator.key();
        msg!("Case 337: establish account with value {}", initial_value);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAccount337<'info> {
    #[account(init, seeds = [b"account", creator.key().as_ref()], bump, payer = creator, space = 8 + 32 + 8)]
    pub record: Account<'info, Record337>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Record337 {
    pub creator: Pubkey,
    pub value: u64,
}
