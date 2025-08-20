use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf260mvTWf");

#[program]
pub mod sync_record_260 {
    use super::*;

    pub fn sync_record(ctx: Context<Ctx260>) -> Result<()> {
        let prev = ctx.accounts.record.identifier;
        let new_id = ctx.accounts.user.key();
        ctx.accounts.record.identifier = new_id;
        msg!("Case 260: identifier {} → {}", prev, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx260<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record260>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Record260 {
    pub matched: Pubkey,
    pub identifier: Pubkey,
}
