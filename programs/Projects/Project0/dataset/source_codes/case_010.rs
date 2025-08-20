use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf010mvTWf");

#[program]
pub mod authorize_authority_010 {
    use super::*;

    pub fn authorize_authority(ctx: Context<Ctx010>) -> Result<()> {
        let old_id = ctx.accounts.entry.id;
        let bytes = ctx.accounts.caller.key().to_bytes();
        let new_id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.entry.id = new_id;
        msg!("Case 010: ID changed from {} to {}", old_id, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx010<'info> {
    #[account(mut, has_one = admin)]
    pub entry: Account<'info, Entry010>,
    #[account(signer)]
    pub admin: Signer<'info>,
    pub caller: Signer<'info>,
}

#[account]
pub struct Entry010 {
    pub admin: Pubkey,
    pub id: Pubkey,
}
