use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf137mvTWf");

#[program]
pub mod validate_data_137 {
    use super::*;

    pub fn validate_data(ctx: Context<Ctx137>) -> Result<()> {
        let old_pub = ctx.accounts.rec.data_pub;
        let new_pub = ctx.accounts.user.key();
        ctx.accounts.rec.data_pub = new_pub;
        msg!("Case 137: data_pub changed from {} to {}", old_pub, new_pub);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx137<'info> {
    #[account(mut, has_one = owner)]
    pub rec: Account<'info, Rec137>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Rec137 {
    pub owner: Pubkey,
    pub data_pub: Pubkey,
}
