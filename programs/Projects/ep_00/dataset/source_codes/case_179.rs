use anchor_lang::prelude::*;

declare_id!("HAI7Z6Lf974ZgDqLU0WWjLp1Qv4d4Oj1C8CsSy1mxKlT");

#[derive(Accounts)]
pub struct Case179<'info> {
    #[account(mut, has_one = owner42)] pub acct26: Account<'info, DataAccount>,
    #[account(mut)] pub acct35: Account<'info, DataAccount>,
    #[account(mut)] pub acct31: Account<'info, DataAccount>,
    pub owner42: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_179_program {
    use super::*;

    pub fn case_179(ctx: Context<Case179>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct26.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct26.data = tripled;
        Ok(())
    }
}
