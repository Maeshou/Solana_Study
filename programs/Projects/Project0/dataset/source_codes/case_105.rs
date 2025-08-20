use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf105mvTWf");

#[program]
pub mod update_data_105 {
    use super::*;

    pub fn update_data(ctx: Context<Ctx105>) -> Result<()> {
        let old_id = ctx.accounts.record.id;
        let bytes = ctx.accounts.initiator.key().to_bytes();
        let new_id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.record.id = new_id;
        msg!("Case 105: id {} -> {}", old_id, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx105<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record105>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub initiator: Signer<'info>,
}

#[account]
pub struct Record105 {
    pub owner: Pubkey,
    pub id: Pubkey,
}
