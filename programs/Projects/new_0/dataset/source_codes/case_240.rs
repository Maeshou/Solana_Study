use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf240mvTWf");

#[program]
pub mod sync_record_240 {
    use super::*;

    pub fn sync_record(ctx: Context<Ctx240>) -> Result<()> {
        let prev = ctx.accounts.record.identifier;
        let new_id = ctx.accounts.user.key();
        ctx.accounts.record.identifier = new_id;
        msg!("Case 240: identifier {} → {}", prev, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx240<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record240>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Record240 {
    pub matched: Pubkey,
    pub identifier: Pubkey,
}
