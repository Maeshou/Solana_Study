use anchor_lang::prelude::*;

declare_id!("FaQjV7anOcIaZJIDR75YhiGtOBqerdddwftA8bfN9Fiv");

#[derive(Accounts)]
pub struct Case150<'info> {
    #[account(mut, has_one = owner29)] pub acct88: Account<'info, DataAccount>,
    #[account(mut)] pub acct15: Account<'info, DataAccount>,
    pub owner29: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_150_program {
    use super::*;

    pub fn case_150(ctx: Context<Case150>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner29.data;
        ctx.accounts.acct88.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
