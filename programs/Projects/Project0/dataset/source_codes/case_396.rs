use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf396mvTWf");

#[program]
pub mod initiate_bucket_396 {
    use super::*;

    pub fn initialize_bucket(ctx: Context<InitializeBucket396>) -> Result<()> {
        // Set authority to the payer
        let authority = ctx.accounts.user.key();
        ctx.accounts.record.authority = authority;
        // Log initialization
        msg!("Case 396: initiate bucket for {}", authority);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeBucket396<'info> {
    #[account(init, seeds = [b"bucket", user.key().as_ref()], bump, payer = user, space = 8 + 32)]
    pub record: Account<'info, Record396>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Record396 {
    pub authority: Pubkey,
}
