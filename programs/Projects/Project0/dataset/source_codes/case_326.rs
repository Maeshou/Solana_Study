use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf326mvTWf");

#[program]
pub mod initiate_bucket_326 {
    use super::*;

    pub fn initialize_bucket(ctx: Context<InitializeBucket326>) -> Result<()> {
        // Set authority to the payer
        let authority = ctx.accounts.user.key();
        ctx.accounts.record.authority = authority;
        // Log initialization
        msg!("Case 326: initiate bucket for {}", authority);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeBucket326<'info> {
    #[account(init, seeds = [b"bucket", user.key().as_ref()], bump, payer = user, space = 8 + 32)]
    pub record: Account<'info, Record326>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Record326 {
    pub authority: Pubkey,
}
