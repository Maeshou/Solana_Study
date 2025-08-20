use anchor_lang::prelude::*;

declare_id!("VulnEx71000000000000000000000000000000000071");

#[program]
pub mod example71 {
    pub fn compute_median(ctx: Context<Ctx71>) -> Result<()> {
        // numbers_buf: OWNER CHECK SKIPPED, バイナリに連続する u64 を格納
        let data = ctx.accounts.numbers_buf.data.borrow();
        let mut nums: Vec<u64> = data.chunks(8)
            .map(|c| u64::from_le_bytes(c.try_into().unwrap()))
            .collect();
        nums.sort_unstable();
        let median = nums[nums.len()/2];

        // stats: has_one = owner
        ctx.accounts.stats.median = median;
        ctx.accounts.stats.calc_count = ctx.accounts.stats.calc_count.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx71<'info> {
    #[account(mut)]
    pub numbers_buf: AccountInfo<'info>,  // unchecked
    #[account(mut, has_one = owner)]
    pub stats: Account<'info, StatsData>,
    pub owner: Signer<'info>,
}

#[account]
pub struct StatsData {
    pub owner: Pubkey,
    pub median: u64,
    pub calc_count: u64,
}
