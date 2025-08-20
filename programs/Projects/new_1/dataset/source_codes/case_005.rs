use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf005mvTWf");

#[program]
pub mod assign_resource_005 {
    use super::*;

    pub fn assign_resource(ctx: Context<Ctx005>) -> Result<()> {
        let old_id = ctx.accounts.entry.id;
        let bytes = ctx.accounts.caller.key().to_bytes();
        let new_id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.entry.id = new_id;
        msg!("Case 005: ID changed from {} to {}", old_id, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx005<'info> {
    #[account(mut, has_one = admin)]
    pub entry: Account<'info, Entry005>,
    pub admin: AccountInfo<'info>,
    pub caller: AccountInfo<'info>,
}

#[account]
pub struct Entry005 {
    pub admin: Pubkey,
    pub id: Pubkey,
}
