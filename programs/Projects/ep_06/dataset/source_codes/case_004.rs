use anchor_lang::prelude::*;
declare_id!("STAK0041111111111111111111111111111111111111");

#[program]
pub mod case004 {
    use super::*;
    pub fn execute_stakingregister(ctx: Context<StakingRegisterContext>) -> Result<()> {
        // Update stake amount
        let mut data = StakeAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        let stake = 500u64;
        data.stake_amount = data.stake_amount.checked_add(stake).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StakingRegisterContext<'info> {
    /// CHECK: expecting StakingRegisterAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting StakingRegisterAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StakingRegisterAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}