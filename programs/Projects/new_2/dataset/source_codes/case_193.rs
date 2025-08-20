use anchor_lang::prelude::*;

declare_id!("OwnChkD4000000000000000000000000000000005");

#[program]
pub mod lending_interest {
    pub fn apply_interest(
        ctx: Context<ApplyInterest>,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        // 属性レベルで manager を検証
        let interest = pool.balance * pool.rate / 10_000;
        pool.balance = pool.balance.saturating_add(interest);
        pool.apply_count = pool.apply_count.saturating_add(1);

        // rate_log は unchecked
        ctx.accounts.rate_log.data.borrow_mut().extend_from_slice(&pool.rate.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ApplyInterest<'info> {
    #[account(mut, has_one = manager)]
    pub pool: Account<'info, LendingPool>,
    pub manager: Signer<'info>,
    /// CHECK: 利率ログ、所有者検証なし
    #[account(mut)]
    pub rate_log: AccountInfo<'info>,
}

#[account]
pub struct LendingPool {
    pub manager: Pubkey,
    pub balance: u64,
    pub rate: u64,
    pub apply_count: u64,
}
