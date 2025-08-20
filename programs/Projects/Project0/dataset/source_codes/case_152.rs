use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf152mvTWf");

#[program]
pub mod amend_account_152 {
    use super::*;

    pub fn amend_account(ctx: Context<Ctx152>) -> Result<()> {
        let old_pub = ctx.accounts.rec.account_pub;
        let new_pub = ctx.accounts.user.key();
        ctx.accounts.rec.account_pub = new_pub;
        msg!("Case 152: account_pub changed from {} to {}", old_pub, new_pub);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx152<'info> {
    #[account(mut, has_one = owner)]
    pub rec: Account<'info, Rec152>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Rec152 {
    pub owner: Pubkey,
    pub account_pub: Pubkey,
}
