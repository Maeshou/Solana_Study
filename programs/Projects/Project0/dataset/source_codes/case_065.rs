use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf065mvTWf");

#[program]
pub mod assign_resource_065 {
    use super::*;

    pub fn assign_resource(ctx: Context<Ctx065>) -> Result<()> {
        let old_id = ctx.accounts.entry.id;
        let bytes = ctx.accounts.caller.key().to_bytes();
        let new_id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.entry.id = new_id;
        msg!("Case 065: ID changed from {} to {}", old_id, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx065<'info> {
    #[account(mut, has_one = admin)]
    pub entry: Account<'info, Entry065>,
    #[account(signer)]
    pub admin: Signer<'info>,
    pub caller: Signer<'info>,
}

#[account]
pub struct Entry065 {
    pub admin: Pubkey,
    pub id: Pubkey,
}
