use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf270mvTWf");

#[program]
pub mod sync_record_270 {
    use super::*;

    pub fn sync_record(ctx: Context<Ctx270>) -> Result<()> {
        let prev = ctx.accounts.record.identifier;
        let new_id = ctx.accounts.user.key();
        ctx.accounts.record.identifier = new_id;
        msg!("Case 270: identifier {} → {}", prev, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx270<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record270>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Record270 {
    pub matched: Pubkey,
    pub identifier: Pubkey,
}
