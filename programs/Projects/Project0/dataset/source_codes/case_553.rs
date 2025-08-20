use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf553mvTWf");

#[program]
pub mod set_entry_553 {
    use super::*;

    pub fn set_entry(ctx: Context<Ctx553>) -> Result<()> {
        let new_id = Pubkey::new(&ctx.accounts.owner.key().to_bytes());
        ctx.accounts.record.id = new_id;
        msg!("Case 553: id set to {}", new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx553<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record553>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record553 {
    pub owner: Pubkey,
    pub id: Pubkey,
}
