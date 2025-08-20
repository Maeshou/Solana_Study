use anchor_lang::prelude::*;

declare_id!("XrMuXEOcL4qqbo8CnFJWFVgnc51PZAi1FLgBiX6205Vc");

#[derive(Accounts)]
pub struct Case167<'info> {
    #[account(mut, has_one = owner41)] pub acct54: Account<'info, DataAccount>,
    #[account(mut)] pub acct96: Account<'info, DataAccount>,
    pub owner41: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_167_program {
    use super::*;

    pub fn case_167(ctx: Context<Case167>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let sub_val = ctx.accounts.acct54.data;
        let result = sub_val.saturating_sub(amount.checked_div(2).unwrap());
        ctx.accounts.acct54.data = result;
        Ok(())
    }
}
