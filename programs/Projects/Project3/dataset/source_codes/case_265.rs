use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf265mvTWf");

#[program]
pub mod link_registry_265 {
    use super::*;

    pub fn link_registry(ctx: Context<Ctx265>) -> Result<()> {
        let prev = ctx.accounts.record.identifier;
        let new_id = ctx.accounts.user.key();
        ctx.accounts.record.identifier = new_id;
        msg!("Case 265: identifier {} → {}", prev, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx265<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record265>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Record265 {
    pub matched: Pubkey,
    pub identifier: Pubkey,
}
