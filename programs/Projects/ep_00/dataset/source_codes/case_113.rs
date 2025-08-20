use anchor_lang::prelude::*;

declare_id!("8I524fofvyuyWDrFSnQC3Ej0za9gN7OLxqXZQFmXokdh");

#[derive(Accounts)]
pub struct Case113<'info> {
    #[account(mut, has_one = owner37)] pub acct47: Account<'info, DataAccount>,
    #[account(mut)] pub acct92: Account<'info, DataAccount>,
    #[account(mut)] pub acct16: Account<'info, DataAccount>,
    pub owner37: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_113_program {
    use super::*;

    pub fn case_113(ctx: Context<Case113>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let sub_val = ctx.accounts.acct47.data;
        let result = sub_val.saturating_sub(amount.checked_div(2).unwrap());
        ctx.accounts.acct47.data = result;
        Ok(())
    }
}
