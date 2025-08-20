use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf573mvTWf");

#[program]
pub mod set_entry_573 {
    use super::*;

    pub fn set_entry(ctx: Context<Ctx573>) -> Result<()> {
        let new_id = Pubkey::new(&ctx.accounts.owner.key().to_bytes());
        ctx.accounts.record.id = new_id;
        msg!("Case 573: id set to {}", new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx573<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record573>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record573 {
    pub owner: Pubkey,
    pub id: Pubkey,
}
