use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf232mvTWf");

#[program]
pub mod sync_data_232 {
    use super::*;

    pub fn sync_data(ctx: Context<Ctx232>) -> Result<()> {
        let prev = ctx.accounts.entry.key_field;
        let new_key = ctx.accounts.user.key();
        ctx.accounts.entry.key_field = new_key;
        msg!("Case 232: key_field {} → {}", prev, new_key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx232<'info> {
    #[account(mut, has_one = matched)]
    pub entry: Account<'info, Entry232>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Entry232 {
    pub matched: Pubkey,
    pub key_field: Pubkey,
}
