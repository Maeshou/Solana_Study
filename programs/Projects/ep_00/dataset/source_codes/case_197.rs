use anchor_lang::prelude::*;

declare_id!("HI8Opnh4oJlyN89B0s5wrSpbPyI9JSlsHQ1F4SjOUkFh");

#[derive(Accounts)]
pub struct Case197<'info> {
    #[account(mut, has_one = owner39)] pub acct70: Account<'info, DataAccount>,
    #[account(mut)] pub acct33: Account<'info, DataAccount>,
    #[account(mut)] pub acct72: Account<'info, DataAccount>,
    pub owner39: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_197_program {
    use super::*;

    pub fn case_197(ctx: Context<Case197>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let sub_val = ctx.accounts.acct70.data;
        let result = sub_val.saturating_sub(amount.checked_div(2).unwrap());
        ctx.accounts.acct70.data = result;
        Ok(())
    }
}
