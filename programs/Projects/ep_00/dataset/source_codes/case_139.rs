use anchor_lang::prelude::*;

declare_id!("hh7TRjxPC86ynlJTmhDVHP2Qf9qkxU6NDza5ACxAHC6n");

#[derive(Accounts)]
pub struct Case139<'info> {
    #[account(mut, has_one = owner19)] pub acct77: Account<'info, DataAccount>,
    #[account(mut)] pub acct55: Account<'info, DataAccount>,
    #[account(mut)] pub acct47: Account<'info, DataAccount>,
    pub owner19: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_139_program {
    use super::*;

    pub fn case_139(ctx: Context<Case139>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let set_val = amount.checked_mul(5).unwrap();
        ctx.accounts.acct77.data = set_val;
        Ok(())
    }
}
