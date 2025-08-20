use anchor_lang::prelude::*;

declare_id!("yUn13S8jWXKATEHW8LZO5dumAmfIBlBUVKHmCm1Talck");

#[derive(Accounts)]
pub struct Case161<'info> {
    #[account(mut, has_one = owner9)] pub acct17: Account<'info, DataAccount>,
    #[account(mut)] pub acct19: Account<'info, DataAccount>,
    #[account(mut)] pub acct23: Account<'info, DataAccount>,
    pub owner9: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_161_program {
    use super::*;

    pub fn case_161(ctx: Context<Case161>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct17.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct17.data = tripled;
        Ok(())
    }
}
