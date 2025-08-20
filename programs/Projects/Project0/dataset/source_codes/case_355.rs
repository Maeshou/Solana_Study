use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf355mvTWf");

#[program]
pub mod forge_repository_355 {
    use super::*;

    pub fn initialize_repository(ctx: Context<InitializeRepository355>) -> Result<()> {
        // Derive and store a new ID
        let bytes = ctx.accounts.user.key().to_bytes();
        let id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.record.id = id;
        msg!("Case 355: forge repository id stored");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeRepository355<'info> {
    #[account(init, seeds = [b"repository", user.key().as_ref()], bump, payer = user, space = 8 + 32 + 32)]
    pub record: Account<'info, Record355>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Record355 {
    pub id: Pubkey,
}
