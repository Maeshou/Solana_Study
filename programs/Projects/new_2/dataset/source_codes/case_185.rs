use anchor_lang::prelude::*;

declare_id!("OwnChkC7000000000000000000000000000000007");

#[program]
pub mod stake_pool {
    pub fn deposit(
        ctx: Context<DepositStake>,
        amount: u64,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        // 属性検証で pool.authority をチェック
        *pool.stakes.entry(ctx.accounts.user.key()).or_insert(0) += amount;
        pool.total_staked = pool.total_staked.saturating_add(amount);

        // validator_log は unchecked
        ctx.accounts.validator_log.data.borrow_mut().extend_from_slice(&amount.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DepositStake<'info> {
    #[account(mut, has_one = authority)]
    pub pool: Account<'info, StakePoolData>,
    pub authority: Signer<'info>,
    pub user: Signer<'info>,
    /// CHECK: バリデータログ、所有者検証なし
    #[account(mut)]
    pub validator_log: AccountInfo<'info>,
}

#[account]
pub struct StakePoolData {
    pub authority: Pubkey,
    pub stakes: std::collections::BTreeMap<Pubkey, u64>,
    pub total_staked: u64,
}
