use anchor_lang::prelude::*;

declare_id!("zhPQQVJZIqLJnmp5zab6aB4dmLaenpJ36D0wrb0onofh");

#[derive(Accounts)]
pub struct Case124<'info> {
    #[account(mut, has_one = owner43)] pub acct29: Account<'info, DataAccount>,
    #[account(mut)] pub acct50: Account<'info, DataAccount>,
    #[account(mut)] pub acct69: Account<'info, DataAccount>,
    pub owner43: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_124_program {
    use super::*;

    pub fn case_124(ctx: Context<Case124>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let sub_val = ctx.accounts.acct29.data;
        let result = sub_val.saturating_sub(amount.checked_div(2).unwrap());
        ctx.accounts.acct29.data = result;
        Ok(())
    }
}
