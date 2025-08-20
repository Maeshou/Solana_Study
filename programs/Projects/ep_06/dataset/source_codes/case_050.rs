use anchor_lang::prelude::*;
declare_id!("JOIN0501111111111111111111111111111111111111");

#[program]
pub mod case050 {
    use super::*;
    pub fn execute_joinrewardpool(ctx: Context<JoinRewardPoolContext>) -> Result<()> {
        // Default context logic
        msg!("Case 50 executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct JoinRewardPoolContext<'info> {
    /// CHECK: expecting JoinRewardPoolAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting JoinRewardPoolAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct JoinRewardPoolAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}