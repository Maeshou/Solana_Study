use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf205mvTWf");

#[program]
pub mod link_registry_205 {
    use super::*;

    pub fn link_registry(ctx: Context<Ctx205>) -> Result<()> {
        let prev = ctx.accounts.record.identifier;
        let new_id = ctx.accounts.user.key();
        ctx.accounts.record.identifier = new_id;
        msg!("Case 205: identifier {} → {}", prev, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx205<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record205>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Record205 {
    pub matched: Pubkey,
    pub identifier: Pubkey,
}
