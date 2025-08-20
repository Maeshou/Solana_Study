use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf508mvTWf");

#[program]
pub mod refresh_data_508 {
    use super::*;

    pub fn refresh_data(ctx: Context<Ctx508>) -> Result<()> {
        let new_id = Pubkey::new(&ctx.accounts.owner.key().to_bytes());
        ctx.accounts.record.id = new_id;
        msg!("Case 508: id set to {}", new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx508<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record508>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record508 {
    pub owner: Pubkey,
    pub id: Pubkey,
}
