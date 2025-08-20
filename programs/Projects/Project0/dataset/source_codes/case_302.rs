use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf302mvTWf");

#[program]
pub mod bootstrap_treasury_302 {
    use super::*;

    pub fn initialize_treasury(ctx: Context<InitializeTreasury302>, initial_value: u64) -> Result<()> {
        // Initialize with a starting value
        ctx.accounts.record.value = initial_value;
        ctx.accounts.record.creator = ctx.accounts.creator.key();
        msg!("Case 302: bootstrap treasury with value {}", initial_value);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeTreasury302<'info> {
    #[account(init, seeds = [b"treasury", creator.key().as_ref()], bump, payer = creator, space = 8 + 32 + 8)]
    pub record: Account<'info, Record302>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Record302 {
    pub creator: Pubkey,
    pub value: u64,
}
