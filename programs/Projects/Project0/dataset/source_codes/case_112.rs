use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf112mvTWf");

#[program]
pub mod amend_account_112 {
    use super::*;

    pub fn amend_account(ctx: Context<Ctx112>) -> Result<()> {
        let old_pub = ctx.accounts.rec.account_pub;
        let new_pub = ctx.accounts.user.key();
        ctx.accounts.rec.account_pub = new_pub;
        msg!("Case 112: account_pub changed from {} to {}", old_pub, new_pub);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx112<'info> {
    #[account(mut, has_one = owner)]
    pub rec: Account<'info, Rec112>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Rec112 {
    pub owner: Pubkey,
    pub account_pub: Pubkey,
}
