use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("FactionRep20202020202020202020202020202020");

#[program]
pub mod faction_reputation {
    use super::*;

    pub fn update_rep(
        ctx: Context<ModifyRep>,
        faction: String,
        delta: i64,
    ) -> Result<()> {
        let r = &mut ctx.accounts.rep;
        let entry = r.map.entry(faction.clone()).or_insert(0);
        *entry = (*entry).saturating_add(delta);
        r.history.push((faction, *entry));
        // 履歴は100件まで保持
        if r.history.len() > 100 {
            r.history.remove(0);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyRep<'info> {
    #[account(mut)]
    pub rep: Account<'info, FactionRepData>,
    pub user: Signer<'info>,
}

#[account]
pub struct FactionRepData {
    pub map: BTreeMap<String, i64>,
    pub history: Vec<(String, i64)>,
}
