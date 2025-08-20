use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf177mvTWf");

#[program]
pub mod validate_data_177 {
    use super::*;

    pub fn validate_data(ctx: Context<Ctx177>) -> Result<()> {
        let old_pub = ctx.accounts.rec.data_pub;
        let new_pub = ctx.accounts.user.key();
        ctx.accounts.rec.data_pub = new_pub;
        msg!("Case 177: data_pub changed from {} to {}", old_pub, new_pub);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx177<'info> {
    #[account(mut, has_one = owner)]
    pub rec: Account<'info, Rec177>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Rec177 {
    pub owner: Pubkey,
    pub data_pub: Pubkey,
}
