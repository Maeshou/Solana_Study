use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf150mvTWf");

#[program]
pub mod configure_account_150 {
    use super::*;

    pub fn configure_account(ctx: Context<Ctx150>) -> Result<()> {
        let old_id = ctx.accounts.record.id;
        let bytes = ctx.accounts.initiator.key().to_bytes();
        let new_id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.record.id = new_id;
        msg!("Case 150: id {} -> {}", old_id, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx150<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record150>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub initiator: Signer<'info>,
}

#[account]
pub struct Record150 {
    pub owner: Pubkey,
    pub id: Pubkey,
}
