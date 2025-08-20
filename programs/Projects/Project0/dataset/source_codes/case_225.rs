use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf225mvTWf");

#[program]
pub mod link_registry_225 {
    use super::*;

    pub fn link_registry(ctx: Context<Ctx225>) -> Result<()> {
        let prev = ctx.accounts.record.identifier;
        let new_id = ctx.accounts.user.key();
        ctx.accounts.record.identifier = new_id;
        msg!("Case 225: identifier {} → {}", prev, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx225<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record225>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Record225 {
    pub matched: Pubkey,
    pub identifier: Pubkey,
}
