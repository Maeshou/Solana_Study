// 4. カウンター＋チェックポイント
use anchor_lang::prelude::*;

declare_id!("Cnt44444444444444444444444444444444");

#[program]
pub mod reinit_counter_v2 {
    use super::*;

    // カウンターを初期化
    pub fn initialize_counter(
        ctx: Context<InitializeCounter>,
    ) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.value = 0;
        // チェックポイントも毎回上書きされる
        let cp = &mut ctx.accounts.checkpoint;
        cp.last_value = 0;
        Ok(())
    }

    // カウンターを増加
    pub fn increment(
        ctx: Context<IncrementCounter>,
    ) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        let old = counter.value;
        counter.value = old + 1;
        // チェックポイントを更新
        let cp = &mut ctx.accounts.checkpoint;
        cp.last_value = old;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(mut)]
    pub counter: Account<'info, CounterData>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IncrementCounter<'info> {
    #[account(mut)]
    pub counter: Account<'info, CounterData>,
    #[account(mut)]
    pub checkpoint: Account<'info, CheckpointData>,
}

#[account]
pub struct CounterData {
    pub value: u64,
}

#[account]
pub struct CheckpointData {
    pub last_value: u64,
}
