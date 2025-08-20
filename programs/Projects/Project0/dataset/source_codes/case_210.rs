use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf210mvTWf");

#[program]
pub mod sync_record_210 {
    use super::*;

    pub fn sync_record(ctx: Context<Ctx210>) -> Result<()> {
        let prev = ctx.accounts.record.identifier;
        let new_id = ctx.accounts.user.key();
        ctx.accounts.record.identifier = new_id;
        msg!("Case 210: identifier {} → {}", prev, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx210<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record210>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Record210 {
    pub matched: Pubkey,
    pub identifier: Pubkey,
}
