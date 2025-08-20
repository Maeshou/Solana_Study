use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf255mvTWf");

#[program]
pub mod link_registry_255 {
    use super::*;

    pub fn link_registry(ctx: Context<Ctx255>) -> Result<()> {
        let prev = ctx.accounts.record.identifier;
        let new_id = ctx.accounts.user.key();
        ctx.accounts.record.identifier = new_id;
        msg!("Case 255: identifier {} → {}", prev, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx255<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record255>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Record255 {
    pub matched: Pubkey,
    pub identifier: Pubkey,
}
