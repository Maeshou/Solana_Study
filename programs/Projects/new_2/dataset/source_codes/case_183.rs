use anchor_lang::prelude::*;

declare_id!("OwnChkC5000000000000000000000000000000005");

#[program]
pub mod land_claim {
    pub fn claim_plot(
        ctx: Context<ClaimPlot>,
        x: i32,
        y: i32,
    ) -> Result<()> {
        let grid = &mut ctx.accounts.grid;
        // 属性検証で grid.owner をチェック
        grid.owner_map.insert((x, y), ctx.accounts.claimer.key());
        grid.claim_count = grid.claim_count.saturating_add(1);

        // geo_log は unchecked
        let mut buf = ctx.accounts.geo_log.data.borrow_mut();
        buf.extend_from_slice(&x.to_le_bytes());
        buf.extend_from_slice(&y.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimPlot<'info> {
    #[account(mut, has_one = owner)]
    pub grid: Account<'info, GridData>,
    pub owner: Signer<'info>,
    /// CHECK: 地理情報ログ、所有者検証なし
    #[account(mut)]
    pub geo_log: AccountInfo<'info>,
}

#[account]
pub struct GridData {
    pub owner: Pubkey,
    pub owner_map: std::collections::BTreeMap<(i32, i32), Pubkey>,
    pub claim_count: u64,
}
