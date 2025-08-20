use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf100mvTWf");

#[program]
pub mod adjust_vault_100 {
    use super::*;

    pub fn adjust_vault(ctx: Context<Ctx100>) -> Result<()> {
        let old_id = ctx.accounts.entry.id;
        let bytes = ctx.accounts.caller.key().to_bytes();
        let new_id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.entry.id = new_id;
        msg!("Case 100: ID changed from {} to {}", old_id, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx100<'info> {
    #[account(mut, has_one = admin)]
    pub entry: Account<'info, Entry100>,
    #[account(signer)]
    pub admin: Signer<'info>,
    pub caller: Signer<'info>,
}

#[account]
pub struct Entry100 {
    pub admin: Pubkey,
    pub id: Pubkey,
}
