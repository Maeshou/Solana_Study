
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct StakeWithdrawCtxtyoy<'info> {
    #[account(mut)] pub stake_account: Account<'info, DataAccount>,
    #[account(mut)] pub staker: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_016 {
    use super::*;

    pub fn stake_withdraw(ctx: Context<StakeWithdrawCtxtyoy>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.stake_account;
        // custom logic for stake_withdraw
        for _ in 0..amount { acct.data += 1; }
        msg!("Executed stake_withdraw logic");
        Ok(())
    }
}
