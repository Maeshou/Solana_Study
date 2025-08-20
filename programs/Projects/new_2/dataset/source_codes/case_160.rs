use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("OwnChkB2000000000000000000000000000000002");

#[program]
pub mod reward_pool {
    pub fn distribute_rewards(ctx: Context<Distribute>) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        // 属性レベルで admin チェック済み
        let per = pool.total_usd / pool.stakers.len() as u64;
        for s in pool.stakers.iter() {
            *pool.paid.entry(*s).or_insert(0) = per;
        }
        pool.distributed = true;

        // audit_log は unchecked
        ctx.accounts.audit_log.data.borrow_mut().extend_from_slice(b"dist;");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Distribute<'info> {
    #[account(mut, has_one = admin)]
    pub pool: Account<'info, RewardPoolData>,
    pub admin: Signer<'info>,
    /// CHECK: 監査ログ、所有者検証なし
    #[account(mut)]
    pub audit_log: AccountInfo<'info>,
}

#[account]
pub struct RewardPoolData {
    pub admin: Pubkey,
    pub total_usd: u64,
    pub stakers: Vec<Pubkey>,
    pub paid: BTreeMap<Pubkey, u64>,
    pub distributed: bool,
}
