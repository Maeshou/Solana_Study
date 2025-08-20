use anchor_lang::prelude::*;

declare_id!("CollctVal41414141414141414141414141414141");

#[program]
pub mod value_collector41 {
    use super::*;

    /// 値をリストに追加
    pub fn add_value(ctx: Context<AddValue>, value: u64) -> Result<()> {
        let col = &mut ctx.accounts.collector;
        col.values.push(value);
        Ok(())
    }

    /// 合計と件数を返す
    pub fn snapshot(ctx: Context<Snapshot>) -> Result<CollectorSnapshot> {
        let col = &ctx.accounts.collector;
        let sum = col.values.iter().copied().sum();
        let count = col.values.len() as u64;
        Ok(CollectorSnapshot { sum, count })
    }
}

#[derive(Accounts)]
pub struct AddValue<'info> {
    #[account(mut)]
    pub collector: Account<'info, CollectorData>,
}

#[derive(Accounts)]
pub struct Snapshot<'info> {
    pub collector: Account<'info, CollectorData>,
}

#[account]
pub struct CollectorData {
    pub values: Vec<u64>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CollectorSnapshot {
    pub sum: u64,
    pub count: u64,
}
