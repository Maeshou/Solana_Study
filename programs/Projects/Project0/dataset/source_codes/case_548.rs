use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf548mvTWf");

#[program]
pub mod refresh_data_548 {
    use super::*;

    pub fn refresh_data(ctx: Context<Ctx548>) -> Result<()> {
        let new_id = Pubkey::new(&ctx.accounts.owner.key().to_bytes());
        ctx.accounts.record.id = new_id;
        msg!("Case 548: id set to {}", new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx548<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record548>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record548 {
    pub owner: Pubkey,
    pub id: Pubkey,
}
