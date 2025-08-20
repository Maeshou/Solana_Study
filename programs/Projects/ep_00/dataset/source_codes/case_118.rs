use anchor_lang::prelude::*;

declare_id!("1lzMMuNtQbdBppvxUzbnP5T7YH7FTQR77qwUht9B0Wwf");

#[derive(Accounts)]
pub struct Case118<'info> {
    #[account(mut, has_one = owner17)] pub acct41: Account<'info, DataAccount>,
    #[account(mut)] pub acct55: Account<'info, DataAccount>,
    #[account(mut)] pub acct28: Account<'info, DataAccount>,
    pub owner17: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_118_program {
    use super::*;

    pub fn case_118(ctx: Context<Case118>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let balance = ctx.accounts.acct41.data;
        let new_balance = balance.checked_add(amount).unwrap();
        ctx.accounts.acct41.data = new_balance;
        Ok(())
    }
}
