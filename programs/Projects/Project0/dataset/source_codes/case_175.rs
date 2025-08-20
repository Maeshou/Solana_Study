use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf175mvTWf");

#[program]
pub mod update_data_175 {
    use super::*;

    pub fn update_data(ctx: Context<Ctx175>) -> Result<()> {
        let old_id = ctx.accounts.record.id;
        let bytes = ctx.accounts.initiator.key().to_bytes();
        let new_id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.record.id = new_id;
        msg!("Case 175: id {} -> {}", old_id, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx175<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record175>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub initiator: Signer<'info>,
}

#[account]
pub struct Record175 {
    pub owner: Pubkey,
    pub id: Pubkey,
}
