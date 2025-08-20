use anchor_lang::prelude::*;

declare_id!("SyncC111111111111111111111111111111111111111");

#[program]
pub mod insecure_sync {
    use super::*;

    pub fn sync_counters(ctx: Context<SyncCounters>) -> Result<()> {
        let ctr = &mut ctx.accounts.target_counter;
        // 複数行にわたる同期間同期処理の例
        ctr.primary = ctr.primary.max(ctr.secondary);
        ctr.secondary = ctr.primary.checked_mul(2).unwrap_or(ctr.secondary);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SyncCounters<'info> {
    #[account(mut)]
    pub target_counter: Account<'info, Counter>,
    /// ここで署名者チェックを追加
    pub actor: Signer<'info>,
}

#[account]
pub struct Counter {
    pub primary: u64,
    pub secondary: u64,
}