use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf002mvTWf");

#[program]
pub mod calibrate_access_002 {
    use super::*;

    pub fn calibrate_access(ctx: Context<Ctx002>) -> Result<()> {
        let previous = ctx.accounts.record.access;
        let new_key = ctx.accounts.new_access.key();
        ctx.accounts.record.access = new_key;
        msg!("Case 002: access updated from {} to {}", previous, new_key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx002<'info> {
    #[account(mut, has_one = manager)]
    pub record: Account<'info, Record002>,
    pub manager: AccountInfo<'info>,
    pub new_access: AccountInfo<'info>,
}

#[account]
pub struct Record002 {
    pub manager: Pubkey,
    pub access: Pubkey,
}
