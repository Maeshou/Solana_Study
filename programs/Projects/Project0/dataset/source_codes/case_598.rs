use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf598mvTWf");

#[program]
pub mod refresh_data_598 {
    use super::*;

    pub fn refresh_data(ctx: Context<Ctx598>) -> Result<()> {
        let new_id = Pubkey::new(&ctx.accounts.owner.key().to_bytes());
        ctx.accounts.record.id = new_id;
        msg!("Case 598: id set to {}", new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx598<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record598>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record598 {
    pub owner: Pubkey,
    pub id: Pubkey,
}
