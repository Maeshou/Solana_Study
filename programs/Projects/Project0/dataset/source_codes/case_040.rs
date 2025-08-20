use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf040mvTWf");

#[program]
pub mod adjust_vault_040 {
    use super::*;

    pub fn adjust_vault(ctx: Context<Ctx040>) -> Result<()> {
        let old_id = ctx.accounts.entry.id;
        let bytes = ctx.accounts.caller.key().to_bytes();
        let new_id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.entry.id = new_id;
        msg!("Case 040: ID changed from {} to {}", old_id, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx040<'info> {
    #[account(mut, has_one = admin)]
    pub entry: Account<'info, Entry040>,
    #[account(signer)]
    pub admin: Signer<'info>,
    pub caller: Signer<'info>,
}

#[account]
pub struct Entry040 {
    pub admin: Pubkey,
    pub id: Pubkey,
}
