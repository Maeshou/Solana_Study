use anchor_lang::prelude::*;

declare_id!("r6GiZRxFcS6nM3yAQVra9ADY8NCthnJBY5dMYvZtsw1q");

#[derive(Accounts)]
pub struct Case137<'info> {
    #[account(mut, has_one = owner42)] pub acct74: Account<'info, DataAccount>,
    #[account(mut)] pub acct9: Account<'info, DataAccount>,
    pub owner42: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_137_program {
    use super::*;

    pub fn case_137(ctx: Context<Case137>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let balance = ctx.accounts.acct74.data;
        let new_balance = balance.checked_add(amount).unwrap();
        ctx.accounts.acct74.data = new_balance;
        Ok(())
    }
}
