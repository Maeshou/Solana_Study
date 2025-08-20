use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf187mvTWf");

#[program]
pub mod validate_data_187 {
    use super::*;

    pub fn validate_data(ctx: Context<Ctx187>) -> Result<()> {
        let old_pub = ctx.accounts.rec.data_pub;
        let new_pub = ctx.accounts.user.key();
        ctx.accounts.rec.data_pub = new_pub;
        msg!("Case 187: data_pub changed from {} to {}", old_pub, new_pub);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx187<'info> {
    #[account(mut, has_one = owner)]
    pub rec: Account<'info, Rec187>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Rec187 {
    pub owner: Pubkey,
    pub data_pub: Pubkey,
}
