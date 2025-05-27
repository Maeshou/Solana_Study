
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClaimRewardsCtxvjsu<'info> {
    #[account(mut)] pub reward_vault: Account<'info, DataAccount>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_017 {
    use super::*;

    pub fn claim_rewards(ctx: Context<ClaimRewardsCtxvjsu>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.reward_vault;
        // custom logic for claim_rewards
        for _ in 0..amount { acct.data += 1; }
        msg!("Executed claim_rewards logic");
        Ok(())
    }
}
