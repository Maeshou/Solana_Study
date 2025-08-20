use anchor_lang::prelude::*;

declare_id!("asdgw4OKDcmDw64bGL3WpGWnK1R47dcpbQISjuNDT0zK");

#[derive(Accounts)]
pub struct Case141<'info> {
    #[account(mut, has_one = owner4)] pub acct54: Account<'info, DataAccount>,
    pub owner4: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_141_program {
    use super::*;

    pub fn case_141(ctx: Context<Case141>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let sub_val = ctx.accounts.acct54.data;
        let result = sub_val.saturating_sub(amount.checked_div(2).unwrap());
        ctx.accounts.acct54.data = result;
        Ok(())
    }
}
