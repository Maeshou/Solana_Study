use anchor_lang::prelude::*;

declare_id!("MGEodnd1pPsZHgjUGJBVnISsqVyrp1EbI5qnXDCbSjvs");

#[derive(Accounts)]
pub struct Case140<'info> {
    #[account(mut, has_one = owner29)] pub acct1: Account<'info, DataAccount>,
    #[account(mut)] pub acct3: Account<'info, DataAccount>,
    #[account(mut)] pub acct82: Account<'info, DataAccount>,
    pub owner29: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_140_program {
    use super::*;

    pub fn case_140(ctx: Context<Case140>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let sub_val = ctx.accounts.acct1.data;
        let result = sub_val.saturating_sub(amount.checked_div(2).unwrap());
        ctx.accounts.acct1.data = result;
        Ok(())
    }
}
