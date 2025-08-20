use anchor_lang::prelude::*;

declare_id!("OwnChkE7000000000000000000000000000000008");

#[program]
pub mod pool_rebalance {
    pub fn rebalance(
        ctx: Context<Rebalance>,
        ratios: Vec<u8>,
    ) -> Result<()> {
        let pr = &mut ctx.accounts.pool;
        // 属性レベルで admin を検証
        pr.ratios = ratios.clone();
        pr.rebalance_count = pr.rebalance_count.saturating_add(1);

        // log_buf は unchecked
        ctx.accounts.log_buf.data.borrow_mut().extend_from_slice(&ratios);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Rebalance<'info> {
    #[account(mut, has_one = admin)]
    pub pool: Account<'info, RebalancePool>,
    pub admin: Signer<'info>,
    /// CHECK: ログバッファ、所有者検証なし
    #[account(mut)]
    pub log_buf: AccountInfo<'info>,
}

#[account]
pub struct RebalancePool {
    pub admin: Pubkey,
    pub ratios: Vec<u8>,
    pub rebalance_count: u64,
}
