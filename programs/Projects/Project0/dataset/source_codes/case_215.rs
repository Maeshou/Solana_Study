use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf215mvTWf");

#[program]
pub mod link_registry_215 {
    use super::*;

    pub fn link_registry(ctx: Context<Ctx215>) -> Result<()> {
        let prev = ctx.accounts.record.identifier;
        let new_id = ctx.accounts.user.key();
        ctx.accounts.record.identifier = new_id;
        msg!("Case 215: identifier {} → {}", prev, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx215<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record215>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Record215 {
    pub matched: Pubkey,
    pub identifier: Pubkey,
}
