use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf015mvTWf");

#[program]
pub mod initialize_permission_015 {
    use super::*;

    pub fn initialize_permission(ctx: Context<Ctx015>) -> Result<()> {
        let old_id = ctx.accounts.entry.id;
        let bytes = ctx.accounts.caller.key().to_bytes();
        let new_id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.entry.id = new_id;
        msg!("Case 015: ID changed from {} to {}", old_id, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx015<'info> {
    #[account(mut, has_one = admin)]
    pub entry: Account<'info, Entry015>,
    #[account(signer)]
    pub admin: Signer<'info>,
    pub caller: Signer<'info>,
}

#[account]
pub struct Entry015 {
    pub admin: Pubkey,
    pub id: Pubkey,
}
