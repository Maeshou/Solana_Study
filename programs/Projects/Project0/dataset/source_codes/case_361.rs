use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf361mvTWf");

#[program]
pub mod spawn_vault_361 {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault361>) -> Result<()> {
        // Set authority to the payer
        let authority = ctx.accounts.user.key();
        ctx.accounts.record.authority = authority;
        // Log initialization
        msg!("Case 361: spawn vault for {}", authority);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault361<'info> {
    #[account(init, seeds = [b"vault", user.key().as_ref()], bump, payer = user, space = 8 + 32)]
    pub record: Account<'info, Record361>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Record361 {
    pub authority: Pubkey,
}
