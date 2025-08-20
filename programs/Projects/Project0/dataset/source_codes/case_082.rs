use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf082mvTWf");

#[program]
pub mod calibrate_access_082 {
    use super::*;

    pub fn calibrate_access(ctx: Context<Ctx082>) -> Result<()> {
        let previous = ctx.accounts.record.access;
        let new_key = ctx.accounts.new_access.key();
        ctx.accounts.record.access = new_key;
        msg!("Case 082: access updated from {} to {}", previous, new_key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx082<'info> {
    #[account(mut, has_one = manager)]
    pub record: Account<'info, Record082>,
    #[account(signer)]
    pub manager: Signer<'info>,
    pub new_access: Signer<'info>,
}

#[account]
pub struct Record082 {
    pub manager: Pubkey,
    pub access: Pubkey,
}
