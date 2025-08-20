use anchor_lang::prelude::*;

declare_id!("V2bgfeBUgQQnaIjJAfKZq79DlkgWWEQbj7hKK1P7xueq");

#[derive(Accounts)]
pub struct Case194<'info> {
    #[account(mut, has_one = owner50)] pub acct31: Account<'info, DataAccount>,
    #[account(mut)] pub acct22: Account<'info, DataAccount>,
    pub owner50: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_194_program {
    use super::*;

    pub fn case_194(ctx: Context<Case194>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner50.data;
        ctx.accounts.acct31.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
