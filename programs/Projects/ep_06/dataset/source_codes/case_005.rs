use anchor_lang::prelude::*;
declare_id!("UNST0051111111111111111111111111111111111111");

#[program]
pub mod case005 {
    use super::*;
    pub fn execute_unstake(ctx: Context<UnstakeContext>) -> Result<()> {
        // Unstake lamports back to user
        let mut data = StakeAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        let unstake = 300u64;
        data.stake_amount = data.stake_amount.checked_sub(unstake).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UnstakeContext<'info> {
    /// CHECK: expecting UnstakeAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting UnstakeAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UnstakeAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}