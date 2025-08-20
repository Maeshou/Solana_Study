use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf280mvTWf");

#[program]
pub mod sync_record_280 {
    use super::*;

    pub fn sync_record(ctx: Context<Ctx280>) -> Result<()> {
        let prev = ctx.accounts.record.identifier;
        let new_id = ctx.accounts.user.key();
        ctx.accounts.record.identifier = new_id;
        msg!("Case 280: identifier {} → {}", prev, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx280<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record280>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Record280 {
    pub matched: Pubkey,
    pub identifier: Pubkey,
}
