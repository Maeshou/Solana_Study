use anchor_lang::prelude::*;

declare_id!("bWRcHNOJJbWxFly0QOXWQ5KlHqeS1H4LMYOgXR1y8DoS");

#[derive(Accounts)]
pub struct Case196<'info> {
    #[account(mut, has_one = owner46)] pub acct88: Account<'info, DataAccount>,
    #[account(mut)] pub acct100: Account<'info, DataAccount>,
    #[account(mut)] pub acct15: Account<'info, DataAccount>,
    pub owner46: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_196_program {
    use super::*;

    pub fn case_196(ctx: Context<Case196>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let balance = ctx.accounts.acct88.data;
        let new_balance = balance.checked_add(amount).unwrap();
        ctx.accounts.acct88.data = new_balance;
        Ok(())
    }
}
