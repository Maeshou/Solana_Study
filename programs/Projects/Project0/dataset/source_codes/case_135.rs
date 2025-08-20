use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf135mvTWf");

#[program]
pub mod update_data_135 {
    use super::*;

    pub fn update_data(ctx: Context<Ctx135>) -> Result<()> {
        let old_id = ctx.accounts.record.id;
        let bytes = ctx.accounts.initiator.key().to_bytes();
        let new_id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.record.id = new_id;
        msg!("Case 135: id {} -> {}", old_id, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx135<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record135>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub initiator: Signer<'info>,
}

#[account]
pub struct Record135 {
    pub owner: Pubkey,
    pub id: Pubkey,
}
