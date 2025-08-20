use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("ComboRec0010010010010010010010010010010010");

#[program]
pub mod combo_record {
    use super::*;

    pub fn record_combo(ctx: Context<RecordCombo>, a: u64, b: u64, child: u64) -> Result<()> {
        let cr = &mut ctx.accounts.combo;
        let inner = cr.map.entry(a).or_insert_with(BTreeMap::new);
        inner.insert(b, child);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RecordCombo<'info> {
    #[account(mut)]
    pub combo: Account<'info, ComboData>,
}

#[account]
pub struct ComboData {
    pub map: BTreeMap<u64, BTreeMap<u64, u64>>,
}
