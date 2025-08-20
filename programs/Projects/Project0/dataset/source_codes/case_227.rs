use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf227mvTWf");

#[program]
pub mod link_account_227 {
    use super::*;

    pub fn link_account(ctx: Context<Ctx227>) -> Result<()> {
        let prev = ctx.accounts.entry.key_field;
        let new_key = ctx.accounts.user.key();
        ctx.accounts.entry.key_field = new_key;
        msg!("Case 227: key_field {} → {}", prev, new_key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx227<'info> {
    #[account(mut, has_one = matched)]
    pub entry: Account<'info, Entry227>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Entry227 {
    pub matched: Pubkey,
    pub key_field: Pubkey,
}
