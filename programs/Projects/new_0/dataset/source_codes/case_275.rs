use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf275mvTWf");

#[program]
pub mod link_registry_275 {
    use super::*;

    pub fn link_registry(ctx: Context<Ctx275>) -> Result<()> {
        let prev = ctx.accounts.record.identifier;
        let new_id = ctx.accounts.user.key();
        ctx.accounts.record.identifier = new_id;
        msg!("Case 275: identifier {} → {}", prev, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx275<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record275>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Record275 {
    pub matched: Pubkey,
    pub identifier: Pubkey,
}
