use anchor_lang::prelude::*;

declare_id!("OwnChkD5000000000000000000000000000000006");

#[program]
pub mod oracle_threshold {
    pub fn set_threshold(
        ctx: Context<SetThres>,
        threshold: u64,
    ) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle;
        // 属性レベルで maintainer を検証
        oracle.threshold = threshold;
        oracle.update_count = oracle.update_count.saturating_add(1);

        // metrics は unchecked
        ctx.accounts.metrics.data.borrow_mut().fill(2);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetThres<'info> {
    #[account(mut, has_one = maintainer)]
    pub oracle: Account<'info, OracleData>,
    pub maintainer: Signer<'info>,
    /// CHECK: メトリクスアカウント、所有者検証なし
    #[account(mut)]
    pub metrics: AccountInfo<'info>,
}

#[account]
pub struct OracleData {
    pub maintainer: Pubkey,
    pub threshold: u64,
    pub update_count: u64,
}
