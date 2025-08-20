use anchor_lang::prelude::*;

declare_id!("ExC07UobQsEPqWrHBdYKyOkWhm0VU106VNi4qa9I08B3");

#[derive(Accounts)]
pub struct Case125<'info> {
    #[account(mut, has_one = owner48)] pub acct73: Account<'info, DataAccount>,
    #[account(mut)] pub acct19: Account<'info, DataAccount>,
    #[account(mut)] pub acct96: Account<'info, DataAccount>,
    pub owner48: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_125_program {
    use super::*;

    pub fn case_125(ctx: Context<Case125>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let set_val = amount.checked_mul(5).unwrap();
        ctx.accounts.acct73.data = set_val;
        Ok(())
    }
}
