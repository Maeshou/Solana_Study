use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf340mvTWf");

#[program]
pub mod construct_depot_340 {
    use super::*;

    pub fn initialize_depot(ctx: Context<InitializeDepot340>) -> Result<()> {
        // Derive and store a new ID
        let bytes = ctx.accounts.user.key().to_bytes();
        let id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.record.id = id;
        msg!("Case 340: construct depot id stored");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeDepot340<'info> {
    #[account(init, seeds = [b"depot", user.key().as_ref()], bump, payer = user, space = 8 + 32 + 32)]
    pub record: Account<'info, Record340>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Record340 {
    pub id: Pubkey,
}
