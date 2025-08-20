use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf212mvTWf");

#[program]
pub mod sync_data_212 {
    use super::*;

    pub fn sync_data(ctx: Context<Ctx212>) -> Result<()> {
        let prev = ctx.accounts.entry.key_field;
        let new_key = ctx.accounts.user.key();
        ctx.accounts.entry.key_field = new_key;
        msg!("Case 212: key_field {} → {}", prev, new_key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx212<'info> {
    #[account(mut, has_one = matched)]
    pub entry: Account<'info, Entry212>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Entry212 {
    pub matched: Pubkey,
    pub key_field: Pubkey,
}
