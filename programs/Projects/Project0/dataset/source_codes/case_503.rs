use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf503mvTWf");

#[program]
pub mod set_entry_503 {
    use super::*;

    pub fn set_entry(ctx: Context<Ctx503>) -> Result<()> {
        let new_id = Pubkey::new(&ctx.accounts.owner.key().to_bytes());
        ctx.accounts.record.id = new_id;
        msg!("Case 503: id set to {}", new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx503<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record503>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record503 {
    pub owner: Pubkey,
    pub id: Pubkey,
}
