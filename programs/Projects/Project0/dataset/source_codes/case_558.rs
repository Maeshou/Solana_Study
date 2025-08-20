use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf558mvTWf");

#[program]
pub mod refresh_data_558 {
    use super::*;

    pub fn refresh_data(ctx: Context<Ctx558>) -> Result<()> {
        let new_id = Pubkey::new(&ctx.accounts.owner.key().to_bytes());
        ctx.accounts.record.id = new_id;
        msg!("Case 558: id set to {}", new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx558<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record558>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record558 {
    pub owner: Pubkey,
    pub id: Pubkey,
}
