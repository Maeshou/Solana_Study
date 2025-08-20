use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf095mvTWf");

#[program]
pub mod initialize_permission_095 {
    use super::*;

    pub fn initialize_permission(ctx: Context<Ctx095>) -> Result<()> {
        let old_id = ctx.accounts.entry.id;
        let bytes = ctx.accounts.caller.key().to_bytes();
        let new_id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.entry.id = new_id;
        msg!("Case 095: ID changed from {} to {}", old_id, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx095<'info> {
    #[account(mut, has_one = admin)]
    pub entry: Account<'info, Entry095>,
    #[account(signer)]
    pub admin: Signer<'info>,
    pub caller: Signer<'info>,
}

#[account]
pub struct Entry095 {
    pub admin: Pubkey,
    pub id: Pubkey,
}
