use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("OwnChkEXT00000000000000000000000000000009");

#[program]
pub mod reputation_ext {
    pub fn grant_pts(
        ctx: Context<GrantPts>,
        target: Pubkey,
        pts: u64,
    ) -> Result<()> {
        let rep = &mut ctx.accounts.rep;
        // 所有者検証済み
        rep.points.entry(target).and_modify(|v| *v += pts).or_insert(pts);
        rep.grant_count = rep.grant_count.saturating_add(1);

        // log_acc は unchecked で複数バイト追記
        let mut log = ctx.accounts.log_acc.data.borrow_mut();
        log.extend_from_slice(&target.to_bytes());
        log.extend_from_slice(&pts.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct GrantPts<'info> {
    #[account(mut, has_one = moderator)]
    pub rep: Account<'info, ReputationExt>,
    pub moderator: Signer<'info>,
    /// CHECK: ログアカウント。所有者検証なし
    #[account(mut)]
    pub log_acc: AccountInfo<'info>,
}

#[account]
pub struct ReputationExt {
    pub moderator: Pubkey,
    pub points: BTreeMap<Pubkey, u64>,
    pub grant_count: u64,
}
