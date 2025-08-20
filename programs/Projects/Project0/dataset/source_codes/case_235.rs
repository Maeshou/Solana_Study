use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf235mvTWf");

#[program]
pub mod link_registry_235 {
    use super::*;

    pub fn link_registry(ctx: Context<Ctx235>) -> Result<()> {
        let prev = ctx.accounts.record.identifier;
        let new_id = ctx.accounts.user.key();
        ctx.accounts.record.identifier = new_id;
        msg!("Case 235: identifier {} → {}", prev, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx235<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record235>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Record235 {
    pub matched: Pubkey,
    pub identifier: Pubkey,
}
