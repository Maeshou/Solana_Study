use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf400mvTWf");

#[program]
pub mod construct_depot_400 {
    use super::*;

    pub fn initialize_depot(ctx: Context<InitializeDepot400>) -> Result<()> {
        // Derive and store a new ID
        let bytes = ctx.accounts.user.key().to_bytes();
        let id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.record.id = id;
        msg!("Case 400: construct depot id stored");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeDepot400<'info> {
    #[account(init, seeds = [b"depot", user.key().as_ref()], bump, payer = user, space = 8 + 32 + 32)]
    pub record: Account<'info, Record400>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Record400 {
    pub id: Pubkey,
}
