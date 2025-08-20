use anchor_lang::prelude::*;

declare_id!("r1mGVEnIABPybXgd4dhpCuLCEUflq5K311bthxY1q4mZ");

#[derive(Accounts)]
pub struct Case160<'info> {
    #[account(mut, has_one = owner20)] pub acct3: Account<'info, DataAccount>,
    #[account(mut)] pub acct87: Account<'info, DataAccount>,
    pub owner20: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_160_program {
    use super::*;

    pub fn case_160(ctx: Context<Case160>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let sub_val = ctx.accounts.acct3.data;
        let result = sub_val.saturating_sub(amount.checked_div(2).unwrap());
        ctx.accounts.acct3.data = result;
        Ok(())
    }
}
