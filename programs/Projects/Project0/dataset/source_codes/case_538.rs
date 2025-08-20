use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf538mvTWf");

#[program]
pub mod refresh_data_538 {
    use super::*;

    pub fn refresh_data(ctx: Context<Ctx538>) -> Result<()> {
        let new_id = Pubkey::new(&ctx.accounts.owner.key().to_bytes());
        ctx.accounts.record.id = new_id;
        msg!("Case 538: id set to {}", new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx538<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record538>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record538 {
    pub owner: Pubkey,
    pub id: Pubkey,
}
