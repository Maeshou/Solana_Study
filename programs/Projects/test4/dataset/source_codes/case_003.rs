// case_003.rs
use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

declare_id!("Safe000000000000000000000000000000000000003");

#[program]
pub mod insecure_sync_v2 {
    use super::*;

    pub fn sync_counters(ctx: Context<SyncCounters>) -> Result<()> {
        let source = &mut ctx.accounts.source_counter;
        let target = &mut ctx.accounts.target_counter;

        // カウンターを同期
        target.count = source.count;
        target.synced_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SyncCounters<'info> {
    // 両方とも mut で受け取り、同一かどうかのチェックなし → Duplicate Mutable Account
    #[account(mut)]
    pub source_counter: Account<'info, Counter>,
    #[account(mut)]
    pub target_counter: Account<'info, Counter>,
    // Signer 型がないため、呼び出し元の署名チェックが一切行われない → missing_signer
}

#[account]
pub struct Counter {
    pub count: u64,
    pub synced_at: i64,
}
