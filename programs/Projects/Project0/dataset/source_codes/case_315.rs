use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf315mvTWf");

#[program]
pub mod forge_repository_315 {
    use super::*;

    pub fn initialize_repository(ctx: Context<InitializeRepository315>) -> Result<()> {
        // Derive and store a new ID
        let bytes = ctx.accounts.user.key().to_bytes();
        let id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.record.id = id;
        msg!("Case 315: forge repository id stored");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeRepository315<'info> {
    #[account(init, seeds = [b"repository", user.key().as_ref()], bump, payer = user, space = 8 + 32 + 32)]
    pub record: Account<'info, Record315>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Record315 {
    pub id: Pubkey,
}
