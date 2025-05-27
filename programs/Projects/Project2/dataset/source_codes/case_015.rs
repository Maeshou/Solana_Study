
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct StakeDepositCtxehfw<'info> {
    #[account(mut)] pub stake_account: Account<'info, DataAccount>,
    #[account(mut)] pub staker: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_015 {
    use super::*;

    pub fn stake_deposit(ctx: Context<StakeDepositCtxehfw>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.stake_account;
        // custom logic for stake_deposit
        assert!(ctx.accounts.stake_account.data > 0); acct.data -= amount;
        msg!("Executed stake_deposit logic");
        Ok(())
    }
}
