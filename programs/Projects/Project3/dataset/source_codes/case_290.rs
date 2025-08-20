use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf290mvTWf");

#[program]
pub mod sync_record_290 {
    use super::*;

    pub fn sync_record(ctx: Context<Ctx290>) -> Result<()> {
        let prev = ctx.accounts.record.identifier;
        let new_id = ctx.accounts.user.key();
        ctx.accounts.record.identifier = new_id;
        msg!("Case 290: identifier {} → {}", prev, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx290<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record290>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Record290 {
    pub matched: Pubkey,
    pub identifier: Pubkey,
}
