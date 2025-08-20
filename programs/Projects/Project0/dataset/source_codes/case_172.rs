use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf172mvTWf");

#[program]
pub mod amend_account_172 {
    use super::*;

    pub fn amend_account(ctx: Context<Ctx172>) -> Result<()> {
        let old_pub = ctx.accounts.rec.account_pub;
        let new_pub = ctx.accounts.user.key();
        ctx.accounts.rec.account_pub = new_pub;
        msg!("Case 172: account_pub changed from {} to {}", old_pub, new_pub);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx172<'info> {
    #[account(mut, has_one = owner)]
    pub rec: Account<'info, Rec172>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Rec172 {
    pub owner: Pubkey,
    pub account_pub: Pubkey,
}
