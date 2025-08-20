use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf030mvTWf");

#[program]
pub mod authorize_authority_030 {
    use super::*;

    pub fn authorize_authority(ctx: Context<Ctx030>) -> Result<()> {
        let old_id = ctx.accounts.entry.id;
        let bytes = ctx.accounts.caller.key().to_bytes();
        let new_id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.entry.id = new_id;
        msg!("Case 030: ID changed from {} to {}", old_id, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx030<'info> {
    #[account(mut, has_one = admin)]
    pub entry: Account<'info, Entry030>,
    #[account(signer)]
    pub admin: Signer<'info>,
    pub caller: Signer<'info>,
}

#[account]
pub struct Entry030 {
    pub admin: Pubkey,
    pub id: Pubkey,
}
