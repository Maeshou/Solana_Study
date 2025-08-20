use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf102mvTWf");

#[program]
pub mod amend_account_102 {
    use super::*;

    pub fn amend_account(ctx: Context<Ctx102>) -> Result<()> {
        let old_pub = ctx.accounts.rec.account_pub;
        let new_pub = ctx.accounts.user.key();
        ctx.accounts.rec.account_pub = new_pub;
        msg!("Case 102: account_pub changed from {} to {}", old_pub, new_pub);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx102<'info> {
    #[account(mut, has_one = owner)]
    pub rec: Account<'info, Rec102>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Rec102 {
    pub owner: Pubkey,
    pub account_pub: Pubkey,
}
