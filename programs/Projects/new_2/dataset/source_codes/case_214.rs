use anchor_lang::prelude::*;

declare_id!("VulnVarX5000000000000000000000000000000005");

#[program]
pub mod example5 {
    pub fn compute_average(ctx: Context<Ctx5>) -> Result<()> {
        // data_buf は unchecked
        let data = ctx.accounts.data_buf.data.borrow();
        let sum: u64 = data.chunks(8)
            .map(|c| u64::from_le_bytes(c.try_into().unwrap()))
            .sum();
        let avg = sum / (data.len() / 8) as u64;
        // stats は has_one 検証済み
        ctx.accounts.stats.avg = avg;
        ctx.accounts.stats.calc_count = ctx.accounts.stats.calc_count.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx5<'info> {
    /// CHECK: データバッファ、所有者検証なし
    #[account(mut)]
    pub data_buf: AccountInfo<'info>,

    #[account(mut, has_one = owner)]
    pub stats: Account<'info, Stats>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Stats {
    pub owner: Pubkey,
    pub avg: u64,
    pub calc_count: u64,
}
