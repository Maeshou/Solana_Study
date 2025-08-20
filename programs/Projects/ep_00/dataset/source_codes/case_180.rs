use anchor_lang::prelude::*;

declare_id!("oegBSHlMotxpEML0sso3LxqhVBiOdyTu8RbgxOpWT6zp");

#[derive(Accounts)]
pub struct Case180<'info> {
    #[account(mut, has_one = owner16)] pub acct52: Account<'info, DataAccount>,
    #[account(mut)] pub acct84: Account<'info, DataAccount>,
    #[account(mut)] pub acct39: Account<'info, DataAccount>,
    pub owner16: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_180_program {
    use super::*;

    pub fn case_180(ctx: Context<Case180>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let balance = ctx.accounts.acct52.data;
        let new_balance = balance.checked_add(amount).unwrap();
        ctx.accounts.acct52.data = new_balance;
        Ok(())
    }
}
