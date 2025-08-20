use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf120mvTWf");

#[program]
pub mod configure_account_120 {
    use super::*;

    pub fn configure_account(ctx: Context<Ctx120>) -> Result<()> {
        let old_id = ctx.accounts.record.id;
        let bytes = ctx.accounts.initiator.key().to_bytes();
        let new_id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.record.id = new_id;
        msg!("Case 120: id {} -> {}", old_id, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx120<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record120>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub initiator: Signer<'info>,
}

#[account]
pub struct Record120 {
    pub owner: Pubkey,
    pub id: Pubkey,
}
