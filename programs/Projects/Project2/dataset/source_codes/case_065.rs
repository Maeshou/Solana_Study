
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClaimRewardCtxsipx<'info> {
    #[account(mut)] pub lottery: Account<'info, DataAccount>,
    #[account(mut)] pub winner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_065 {
    use super::*;

    pub fn claim_reward(ctx: Context<ClaimRewardCtxsipx>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.lottery;
        // custom logic for claim_reward
        assert!(ctx.accounts.lottery.data > 0); acct.data -= amount;
        msg!("Executed claim_reward logic");
        Ok(())
    }
}
