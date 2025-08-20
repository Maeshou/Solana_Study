use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf250mvTWf");

#[program]
pub mod sync_record_250 {
    use super::*;

    pub fn sync_record(ctx: Context<Ctx250>) -> Result<()> {
        let prev = ctx.accounts.record.identifier;
        let new_id = ctx.accounts.user.key();
        ctx.accounts.record.identifier = new_id;
        msg!("Case 250: identifier {} → {}", prev, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx250<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record250>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Record250 {
    pub matched: Pubkey,
    pub identifier: Pubkey,
}
