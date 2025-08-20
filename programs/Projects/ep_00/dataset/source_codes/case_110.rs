use anchor_lang::prelude::*;

declare_id!("6EcsXF4bP0qFeZQyBKIgmTdGniuAlpIO9n3LJqJiI7rV");

#[derive(Accounts)]
pub struct Case110<'info> {
    #[account(mut, has_one = owner36)] pub acct5: Account<'info, DataAccount>,
    #[account(mut)] pub acct26: Account<'info, DataAccount>,
    pub owner36: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_110_program {
    use super::*;

    pub fn case_110(ctx: Context<Case110>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner36.data;
        ctx.accounts.acct5.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
