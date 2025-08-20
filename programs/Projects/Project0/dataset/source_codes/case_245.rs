use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf245mvTWf");

#[program]
pub mod link_registry_245 {
    use super::*;

    pub fn link_registry(ctx: Context<Ctx245>) -> Result<()> {
        let prev = ctx.accounts.record.identifier;
        let new_id = ctx.accounts.user.key();
        ctx.accounts.record.identifier = new_id;
        msg!("Case 245: identifier {} → {}", prev, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx245<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record245>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Record245 {
    pub matched: Pubkey,
    pub identifier: Pubkey,
}
