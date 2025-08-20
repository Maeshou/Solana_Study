use anchor_lang::prelude::*;

declare_id!("waUgoPKx0nnNXbj5ProazINqJB5cHUz8e22o8zRUKlnl");

#[derive(Accounts)]
pub struct Case172<'info> {
    #[account(mut, has_one = owner37)] pub acct67: Account<'info, DataAccount>,
    #[account(mut)] pub acct56: Account<'info, DataAccount>,
    #[account(mut)] pub acct62: Account<'info, DataAccount>,
    pub owner37: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_172_program {
    use super::*;

    pub fn case_172(ctx: Context<Case172>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let sub_val = ctx.accounts.acct67.data;
        let result = sub_val.saturating_sub(amount.checked_div(2).unwrap());
        ctx.accounts.acct67.data = result;
        Ok(())
    }
}
