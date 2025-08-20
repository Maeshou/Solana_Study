use anchor_lang::prelude::*;

declare_id!("DynamicFee0077777777777777777777777777777777");

#[program]
pub mod dynamic_fee {
    use super::*;

    pub fn record_volume(ctx: Context<RecordVol>, volume: u64) -> Result<()> {
        let df = &mut ctx.accounts.fee;
        df.buffer[df.index as usize] = volume;
        df.index = (df.index + 1) % (df.buffer.len() as u8);
        df.count = df.count.saturating_add(1).min(df.buffer.len() as u64);
        let sum: u128 = df.buffer.iter().take(df.count as usize).map(|&v| v as u128).sum();
        df.current_fee = ((sum / df.count as u128) as u64) / df.divisor;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RecordVol<'info> {
    #[account(mut)]
    pub fee: Account<'info, FeeData>,
}

#[account]
pub struct FeeData {
    pub buffer: [u64; 10],
    pub index: u8,
    pub count: u64,
    pub divisor: u64,
    pub current_fee: u64,
}
