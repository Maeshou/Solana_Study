use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf237mvTWf");

#[program]
pub mod link_account_237 {
    use super::*;

    pub fn link_account(ctx: Context<Ctx237>) -> Result<()> {
        let prev = ctx.accounts.entry.key_field;
        let new_key = ctx.accounts.user.key();
        ctx.accounts.entry.key_field = new_key;
        msg!("Case 237: key_field {} → {}", prev, new_key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx237<'info> {
    #[account(mut, has_one = matched)]
    pub entry: Account<'info, Entry237>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Entry237 {
    pub matched: Pubkey,
    pub key_field: Pubkey,
}
