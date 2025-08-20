use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf140mvTWf");

#[program]
pub mod configure_account_140 {
    use super::*;

    pub fn configure_account(ctx: Context<Ctx140>) -> Result<()> {
        let old_id = ctx.accounts.record.id;
        let bytes = ctx.accounts.initiator.key().to_bytes();
        let new_id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.record.id = new_id;
        msg!("Case 140: id {} -> {}", old_id, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx140<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record140>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub initiator: Signer<'info>,
}

#[account]
pub struct Record140 {
    pub owner: Pubkey,
    pub id: Pubkey,
}
