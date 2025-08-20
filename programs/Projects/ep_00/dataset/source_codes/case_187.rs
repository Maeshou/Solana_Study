use anchor_lang::prelude::*;

declare_id!("1S7ffTCW1todi4W2PlBljnsHiQz409BG5L8jVUdcR6RL");

#[derive(Accounts)]
pub struct Case187<'info> {
    #[account(mut, has_one = owner10)] pub acct46: Account<'info, DataAccount>,
    pub owner10: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_187_program {
    use super::*;

    pub fn case_187(ctx: Context<Case187>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner10.data;
        ctx.accounts.acct46.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
