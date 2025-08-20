use anchor_lang::prelude::*;

declare_id!("iOdCqyYQ4jpFyyYdc6ndt5IUsCDwF26jTMDKp5K4UyuK");

#[derive(Accounts)]
pub struct Case102<'info> {
    #[account(mut, has_one = owner46)] pub acct14: Account<'info, DataAccount>,
    #[account(mut)] pub acct10: Account<'info, DataAccount>,
    #[account(mut)] pub acct18: Account<'info, DataAccount>,
    pub owner46: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_102_program {
    use super::*;

    pub fn case_102(ctx: Context<Case102>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let set_val = amount.checked_mul(5).unwrap();
        ctx.accounts.acct14.data = set_val;
        Ok(())
    }
}
