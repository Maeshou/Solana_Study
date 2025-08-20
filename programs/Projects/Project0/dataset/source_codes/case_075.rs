use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf075mvTWf");

#[program]
pub mod initialize_permission_075 {
    use super::*;

    pub fn initialize_permission(ctx: Context<Ctx075>) -> Result<()> {
        let old_id = ctx.accounts.entry.id;
        let bytes = ctx.accounts.caller.key().to_bytes();
        let new_id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.entry.id = new_id;
        msg!("Case 075: ID changed from {} to {}", old_id, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx075<'info> {
    #[account(mut, has_one = admin)]
    pub entry: Account<'info, Entry075>,
    #[account(signer)]
    pub admin: Signer<'info>,
    pub caller: Signer<'info>,
}

#[account]
pub struct Entry075 {
    pub admin: Pubkey,
    pub id: Pubkey,
}
