use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf020mvTWf");

#[program]
pub mod adjust_vault_020 {
    use super::*;

    pub fn adjust_vault(ctx: Context<Ctx020>) -> Result<()> {
        let old_id = ctx.accounts.entry.id;
        let bytes = ctx.accounts.caller.key().to_bytes();
        let new_id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.entry.id = new_id;
        msg!("Case 020: ID changed from {} to {}", old_id, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx020<'info> {
    #[account(mut, has_one = admin)]
    pub entry: Account<'info, Entry020>,
    #[account(signer)]
    pub admin: Signer<'info>,
    pub caller: Signer<'info>,
}

#[account]
pub struct Entry020 {
    pub admin: Pubkey,
    pub id: Pubkey,
}
