use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf070mvTWf");

#[program]
pub mod authorize_authority_070 {
    use super::*;

    pub fn authorize_authority(ctx: Context<Ctx070>) -> Result<()> {
        let old_id = ctx.accounts.entry.id;
        let bytes = ctx.accounts.caller.key().to_bytes();
        let new_id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.entry.id = new_id;
        msg!("Case 070: ID changed from {} to {}", old_id, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx070<'info> {
    #[account(mut, has_one = admin)]
    pub entry: Account<'info, Entry070>,
    #[account(signer)]
    pub admin: Signer<'info>,
    pub caller: Signer<'info>,
}

#[account]
pub struct Entry070 {
    pub admin: Pubkey,
    pub id: Pubkey,
}
