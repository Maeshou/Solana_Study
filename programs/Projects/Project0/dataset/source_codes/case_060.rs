use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf060mvTWf");

#[program]
pub mod adjust_vault_060 {
    use super::*;

    pub fn adjust_vault(ctx: Context<Ctx060>) -> Result<()> {
        let old_id = ctx.accounts.entry.id;
        let bytes = ctx.accounts.caller.key().to_bytes();
        let new_id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.entry.id = new_id;
        msg!("Case 060: ID changed from {} to {}", old_id, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx060<'info> {
    #[account(mut, has_one = admin)]
    pub entry: Account<'info, Entry060>,
    #[account(signer)]
    pub admin: Signer<'info>,
    pub caller: Signer<'info>,
}

#[account]
pub struct Entry060 {
    pub admin: Pubkey,
    pub id: Pubkey,
}
