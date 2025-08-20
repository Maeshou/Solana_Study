use anchor_lang::prelude::*;

declare_id!("IyUDOa3YJfKfgUMUjbXjJ0JQqXs9tJSSOjZdRg8rTs7Q");

#[derive(Accounts)]
pub struct Case108<'info> {
    #[account(mut, has_one = owner2)] pub acct22: Account<'info, DataAccount>,
    #[account(mut)] pub acct100: Account<'info, DataAccount>,
    pub owner2: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_108_program {
    use super::*;

    pub fn case_108(ctx: Context<Case108>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner2.data;
        ctx.accounts.acct22.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
