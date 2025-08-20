use anchor_lang::prelude::*;

declare_id!("orjPky3mtJ3fo8Yh6fOEskJ8NI61ob2hAd1ayRp3RJ23");

#[derive(Accounts)]
pub struct Case116<'info> {
    #[account(mut, has_one = owner31)] pub acct100: Account<'info, DataAccount>,
    #[account(mut)] pub acct26: Account<'info, DataAccount>,
    pub owner31: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_116_program {
    use super::*;

    pub fn case_116(ctx: Context<Case116>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner31.data;
        ctx.accounts.acct100.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
