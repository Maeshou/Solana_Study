use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf080mvTWf");

#[program]
pub mod adjust_vault_080 {
    use super::*;

    pub fn adjust_vault(ctx: Context<Ctx080>) -> Result<()> {
        let old_id = ctx.accounts.entry.id;
        let bytes = ctx.accounts.caller.key().to_bytes();
        let new_id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.entry.id = new_id;
        msg!("Case 080: ID changed from {} to {}", old_id, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx080<'info> {
    #[account(mut, has_one = admin)]
    pub entry: Account<'info, Entry080>,
    #[account(signer)]
    pub admin: Signer<'info>,
    pub caller: Signer<'info>,
}

#[account]
pub struct Entry080 {
    pub admin: Pubkey,
    pub id: Pubkey,
}
