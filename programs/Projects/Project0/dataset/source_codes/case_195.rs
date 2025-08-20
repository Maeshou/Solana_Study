use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf195mvTWf");

#[program]
pub mod update_data_195 {
    use super::*;

    pub fn update_data(ctx: Context<Ctx195>) -> Result<()> {
        let old_id = ctx.accounts.record.id;
        let bytes = ctx.accounts.initiator.key().to_bytes();
        let new_id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.record.id = new_id;
        msg!("Case 195: id {} -> {}", old_id, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx195<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record195>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub initiator: Signer<'info>,
}

#[account]
pub struct Record195 {
    pub owner: Pubkey,
    pub id: Pubkey,
}
