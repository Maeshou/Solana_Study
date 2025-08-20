use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgqMetRec001");

#[program]
pub mod insecure_metrics_recorder {
    use super::*;

    /// 各種システム負荷指標を記録する（オーナーチェックを怠っている！）
    pub fn record_metrics(
        ctx: Context<RecordMetrics>,
        cpu_load: u8,
        memory_mb: u16,
        network_kbps: u32,
    ) -> Result<()> {
        // ★ metrics_account.owner == program_id の確認をしていないため、
        //    攻撃者は任意のアカウントを渡してデータを上書きできる
        let mut metrics = Metrics::try_from_slice(&ctx.accounts.metrics_account.data.borrow())?;

        // CPU 使用率（％）を更新
        metrics.cpu_usage = cpu_load;
        // メモリ使用量（MB）を更新
        metrics.mem_usage = memory_mb;
        // ネットワーク使用量（kbps）を更新
        metrics.net_usage = network_kbps;

        // 全フィールドを書き戻し
        metrics.serialize(&mut &mut ctx.accounts.metrics_account.data.borrow_mut()[..])?;

        msg!(
            "Metrics updated → CPU: {}%, MEM: {}MB, NET: {}kbps",
            metrics.cpu_usage,
            metrics.mem_usage,
            metrics.net_usage
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RecordMetrics<'info> {
    /// CHECK: owner チェックをせずに受け取っている生の AccountInfo
    #[account(mut)]
    pub metrics_account: AccountInfo<'info>,

    /// 操作実行者の署名のみを検証
    pub signer: Signer<'info>,
}

#[account]
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Metrics {
    /// CPU 使用率（0～100）
    pub cpu_usage: u8,
    /// メモリ使用量（MB）
    pub mem_usage: u16,
    /// ネットワーク使用量（kbps）
    pub net_usage: u32,
}
