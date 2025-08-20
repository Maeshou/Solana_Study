use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf287mvTWf");

#[program]
pub mod link_account_287 {
    use super::*;

    pub fn link_account(ctx: Context<Ctx287>) -> Result<()> {
        let prev = ctx.accounts.entry.key_field;
        let new_key = ctx.accounts.user.key();
        ctx.accounts.entry.key_field = new_key;
        msg!("Case 287: key_field {} → {}", prev, new_key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx287<'info> {
    #[account(mut, has_one = matched)]
    pub entry: Account<'info, Entry287>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Entry287 {
    pub matched: Pubkey,
    pub key_field: Pubkey,
}
