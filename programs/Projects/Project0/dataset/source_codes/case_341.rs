use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf341mvTWf");

#[program]
pub mod spawn_vault_341 {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault341>) -> Result<()> {
        // Set authority to the payer
        let authority = ctx.accounts.user.key();
        ctx.accounts.record.authority = authority;
        // Log initialization
        msg!("Case 341: spawn vault for {}", authority);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault341<'info> {
    #[account(init, seeds = [b"vault", user.key().as_ref()], bump, payer = user, space = 8 + 32)]
    pub record: Account<'info, Record341>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Record341 {
    pub authority: Pubkey,
}
