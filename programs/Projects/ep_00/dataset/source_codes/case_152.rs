use anchor_lang::prelude::*;

declare_id!("rGXbzO1Aq4No2HimyiysehwrL9hEUdjU4NNBKDnCLg2N");

#[derive(Accounts)]
pub struct Case152<'info> {
    #[account(mut, has_one = owner46)] pub acct47: Account<'info, DataAccount>,
    #[account(mut)] pub acct83: Account<'info, DataAccount>,
    pub owner46: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_152_program {
    use super::*;

    pub fn case_152(ctx: Context<Case152>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let sub_val = ctx.accounts.acct47.data;
        let result = sub_val.saturating_sub(amount.checked_div(2).unwrap());
        ctx.accounts.acct47.data = result;
        Ok(())
    }
}
