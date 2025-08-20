use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf300mvTWf");

#[program]
pub mod sync_record_300 {
    use super::*;

    pub fn sync_record(ctx: Context<Ctx300>) -> Result<()> {
        let prev = ctx.accounts.record.identifier;
        let new_id = ctx.accounts.user.key();
        ctx.accounts.record.identifier = new_id;
        msg!("Case 300: identifier {} → {}", prev, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx300<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record300>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Record300 {
    pub matched: Pubkey,
    pub identifier: Pubkey,
}
