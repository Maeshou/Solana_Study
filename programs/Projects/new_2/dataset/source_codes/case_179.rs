use anchor_lang::prelude::*;

declare_id!("OwnChkC1000000000000000000000000000000001");

#[program]
pub mod loyalty {
    pub fn award_points(
        ctx: Context<AwardPoints>,
        points: u64,
    ) -> Result<()> {
        let acct = &mut ctx.accounts.loyalty;
        // has_one でオーナー検証
        acct.total_points = acct.total_points.saturating_add(points);
        acct.award_count = acct.award_count.saturating_add(1);

        // stats_log は unchecked で追記
        let mut buf = ctx.accounts.stats_log.data.borrow_mut();
        buf.extend_from_slice(&points.to_le_bytes());
        buf.extend_from_slice(&acct.award_count.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AwardPoints<'info> {
    #[account(mut, has_one = owner)]
    pub loyalty: Account<'info, LoyaltyAccount>,
    pub owner: Signer<'info>,
    /// CHECK: 統計ログ用アカウント、所有者検証なし
    #[account(mut)]
    pub stats_log: AccountInfo<'info>,
}

#[account]
pub struct LoyaltyAccount {
    pub owner: Pubkey,
    pub total_points: u64,
    pub award_count: u64,
}
