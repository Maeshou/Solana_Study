use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf588mvTWf");

#[program]
pub mod refresh_data_588 {
    use super::*;

    pub fn refresh_data(ctx: Context<Ctx588>) -> Result<()> {
        let new_id = Pubkey::new(&ctx.accounts.owner.key().to_bytes());
        ctx.accounts.record.id = new_id;
        msg!("Case 588: id set to {}", new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx588<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record588>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record588 {
    pub owner: Pubkey,
    pub id: Pubkey,
}
