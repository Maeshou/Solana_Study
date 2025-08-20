use anchor_lang::prelude::*;

declare_id!("xYxZKGViJ5T9JGCBYG6QH5LgE7BzI0oVXX4cFbf172KD");

#[derive(Accounts)]
pub struct Case120<'info> {
    #[account(mut, has_one = owner42)] pub acct35: Account<'info, DataAccount>,
    #[account(mut)] pub acct84: Account<'info, DataAccount>,
    pub owner42: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_120_program {
    use super::*;

    pub fn case_120(ctx: Context<Case120>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let set_val = amount.checked_mul(5).unwrap();
        ctx.accounts.acct35.data = set_val;
        Ok(())
    }
}
