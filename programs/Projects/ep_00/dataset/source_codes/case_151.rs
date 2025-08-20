use anchor_lang::prelude::*;

declare_id!("VlGGU5LChx6vtI1vZvLKaWn19U9nYOr5jP3YQleJTFII");

#[derive(Accounts)]
pub struct Case151<'info> {
    #[account(mut, has_one = owner40)] pub acct95: Account<'info, DataAccount>,
    #[account(mut)] pub acct39: Account<'info, DataAccount>,
    #[account(mut)] pub acct98: Account<'info, DataAccount>,
    pub owner40: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_151_program {
    use super::*;

    pub fn case_151(ctx: Context<Case151>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let sub_val = ctx.accounts.acct95.data;
        let result = sub_val.saturating_sub(amount.checked_div(2).unwrap());
        ctx.accounts.acct95.data = result;
        Ok(())
    }
}
