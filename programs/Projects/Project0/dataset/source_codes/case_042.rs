use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf042mvTWf");

#[program]
pub mod calibrate_access_042 {
    use super::*;

    pub fn calibrate_access(ctx: Context<Ctx042>) -> Result<()> {
        let previous = ctx.accounts.record.access;
        let new_key = ctx.accounts.new_access.key();
        ctx.accounts.record.access = new_key;
        msg!("Case 042: access updated from {} to {}", previous, new_key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx042<'info> {
    #[account(mut, has_one = manager)]
    pub record: Account<'info, Record042>,
    #[account(signer)]
    pub manager: Signer<'info>,
    pub new_access: Signer<'info>,
}

#[account]
pub struct Record042 {
    pub manager: Pubkey,
    pub access: Pubkey,
}
