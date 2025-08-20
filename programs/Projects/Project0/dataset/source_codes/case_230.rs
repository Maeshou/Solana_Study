use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf230mvTWf");

#[program]
pub mod sync_record_230 {
    use super::*;

    pub fn sync_record(ctx: Context<Ctx230>) -> Result<()> {
        let prev = ctx.accounts.record.identifier;
        let new_id = ctx.accounts.user.key();
        ctx.accounts.record.identifier = new_id;
        msg!("Case 230: identifier {} → {}", prev, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx230<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record230>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Record230 {
    pub matched: Pubkey,
    pub identifier: Pubkey,
}
