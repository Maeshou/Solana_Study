use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf353mvTWf");

#[program]
pub mod originate_cache_353 {
    use super::*;

    pub fn initialize_cache(ctx: Context<InitializeCache353>, seed_num: u64) -> Result<()> {
        // Rotate seed number and store
        let rotated = seed_num.rotate_left((seed_num % 32) as u32);
        ctx.accounts.record.data = rotated;
        msg!("Case 353: originate cache rotated {} to {}", seed_num, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCache353<'info> {
    #[account(init, seeds = [b"cache", user.key().as_ref()], bump, payer = user, space = 8 + 32 + 8)]
    pub record: Account<'info, Record353>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Record353 {
    pub payer: Pubkey,
    pub data: u64,
}
