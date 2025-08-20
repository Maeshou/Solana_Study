use anchor_lang::prelude::*;

declare_id!("JQBoJQxTUKw80JWALeY7m3Ns1GHqqnc1ty5fQQbTPIj9");

#[derive(Accounts)]
pub struct Case154<'info> {
    #[account(mut, has_one = owner30)] pub acct26: Account<'info, DataAccount>,
    #[account(mut)] pub acct93: Account<'info, DataAccount>,
    #[account(mut)] pub acct99: Account<'info, DataAccount>,
    pub owner30: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_154_program {
    use super::*;

    pub fn case_154(ctx: Context<Case154>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct26.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct26.data = tripled;
        Ok(())
    }
}
