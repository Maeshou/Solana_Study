use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("PartExa8888888888888888888888888888888888");

#[program]
pub mod participation_extra {
    use super::*;

    pub fn register(ctx: Context<ModifyPart>, player: Pubkey) -> Result<()> {
        let p = &mut ctx.accounts.part;
        if p.counts.get(&player).unwrap_or(&0) < &5u64 {
            p.counts.insert(player, p.counts.get(&player).unwrap_or(&0).saturating_add(1));
        } else {
            // 上限超過ならバウンスバック
            p.bounced.push(player);
            p.bounce_count = p.bounce_count.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyPart<'info> {
    #[account(mut)]
    pub part: Account<'info, PartExtraData>,
}

#[account]
pub struct PartExtraData {
    pub counts: BTreeMap<Pubkey, u64>,
    pub bounced: Vec<Pubkey>,
    pub bounce_count: u64,
}
