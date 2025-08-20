use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf197mvTWf");

#[program]
pub mod validate_data_197 {
    use super::*;

    pub fn validate_data(ctx: Context<Ctx197>) -> Result<()> {
        let old_pub = ctx.accounts.rec.data_pub;
        let new_pub = ctx.accounts.user.key();
        ctx.accounts.rec.data_pub = new_pub;
        msg!("Case 197: data_pub changed from {} to {}", old_pub, new_pub);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx197<'info> {
    #[account(mut, has_one = owner)]
    pub rec: Account<'info, Rec197>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Rec197 {
    pub owner: Pubkey,
    pub data_pub: Pubkey,
}
