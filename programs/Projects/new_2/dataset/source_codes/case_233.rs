use anchor_lang::prelude::*;

declare_id!("VulnEx31000000000000000000000000000000000031");

#[program]
pub mod example31 {
    pub fn distribute_rewards(ctx: Context<Ctx31>, amount: u64) -> Result<()> {
        // reward_log は所有者検証なし
        ctx.accounts.reward_log.data.borrow_mut().extend_from_slice(&amount.to_le_bytes());
        // reward_pool は has_one で admin 検証済み
        let pool = &mut ctx.accounts.reward_pool;
        pool.total_distributed = pool.total_distributed.saturating_add(amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx31<'info> {
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub reward_log: AccountInfo<'info>,
    #[account(mut, has_one = admin)]
    pub reward_pool: Account<'info, RewardPool>,
    pub admin: Signer<'info>,
}

#[account]
pub struct RewardPool {
    pub admin: Pubkey,
    pub total_distributed: u64,
}
