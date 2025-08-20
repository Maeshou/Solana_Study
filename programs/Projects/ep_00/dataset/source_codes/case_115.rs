use anchor_lang::prelude::*;

declare_id!("MGunSWHxgr86vDSvX27KJEoHacPOleMwLnnpFdKUy7Gs");

#[derive(Accounts)]
pub struct Case115<'info> {
    #[account(mut, has_one = owner13)] pub acct57: Account<'info, DataAccount>,
    pub owner13: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_115_program {
    use super::*;

    pub fn case_115(ctx: Context<Case115>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner13.data;
        ctx.accounts.acct57.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
