use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf222mvTWf");

#[program]
pub mod sync_data_222 {
    use super::*;

    pub fn sync_data(ctx: Context<Ctx222>) -> Result<()> {
        let prev = ctx.accounts.entry.key_field;
        let new_key = ctx.accounts.user.key();
        ctx.accounts.entry.key_field = new_key;
        msg!("Case 222: key_field {} → {}", prev, new_key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx222<'info> {
    #[account(mut, has_one = matched)]
    pub entry: Account<'info, Entry222>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Entry222 {
    pub matched: Pubkey,
    pub key_field: Pubkey,
}
