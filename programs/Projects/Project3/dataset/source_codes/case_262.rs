use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf262mvTWf");

#[program]
pub mod sync_data_262 {
    use super::*;

    pub fn sync_data(ctx: Context<Ctx262>) -> Result<()> {
        let prev = ctx.accounts.entry.key_field;
        let new_key = ctx.accounts.user.key();
        ctx.accounts.entry.key_field = new_key;
        msg!("Case 262: key_field {} → {}", prev, new_key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx262<'info> {
    #[account(mut, has_one = matched)]
    pub entry: Account<'info, Entry262>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Entry262 {
    pub matched: Pubkey,
    pub key_field: Pubkey,
}
