use anchor_lang::prelude::*;

declare_id!("FFiDw5v4LqzG02r0E4ttjRo9Iim5nFrzzrBBauAA1fa6");

#[derive(Accounts)]
pub struct Case104<'info> {
    #[account(mut, has_one = owner42)] pub acct49: Account<'info, DataAccount>,
    pub owner42: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_104_program {
    use super::*;

    pub fn case_104(ctx: Context<Case104>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let sub_val = ctx.accounts.acct49.data;
        let result = sub_val.saturating_sub(amount.checked_div(2).unwrap());
        ctx.accounts.acct49.data = result;
        Ok(())
    }
}
