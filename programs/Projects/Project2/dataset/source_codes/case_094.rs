use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("AccCount43434343434343434343434343434343");

#[program]
pub mod access_counter43 {
    use super::*;

    /// 呼び出し者の回数をインクリメント
    pub fn record(ctx: Context<ModifyCounter>) -> Result<()> {
        let ctr = &mut ctx.accounts.counter;
        let key = ctx.accounts.user.key();
        let entry = ctr.map.entry(key).or_insert(0);
        *entry = entry.saturating_add(1);
        Ok(())
    }

    /// 現在のマップを返す
    pub fn get_counts(ctx: Context<ViewCounter>) -> Result<CountView> {
        let ctr = &ctx.accounts.counter;
        Ok(CountView { map: ctr.map.clone() })
    }
}

#[derive(Accounts)]
pub struct ModifyCounter<'info> {
    #[account(mut)]
    pub counter: Account<'info, CounterData>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ViewCounter<'info> {
    pub counter: Account<'info, CounterData>,
}

#[account]
pub struct CounterData {
    pub map: BTreeMap<Pubkey, u64>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CountView {
    pub map: BTreeMap<Pubkey, u64>,
}
