use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgqTimeLck");

#[program]
pub mod insecure_timelock {
    use super::*;

    /// タイムロック解除時刻を延長する（Owner Check をまったく行っていない！）
    pub fn extend_timelock(ctx: Context<ExtendTimeLock>, extra_secs: u64) -> Result<()> {
        let acct = &mut ctx.accounts.timelock_account.to_account_info();
        let mut data = acct.data.borrow_mut();

        // ★ owner==program_id の検証が抜けているため、別プログラム所有のアカウントでも通過してしまう
        // 先頭 8 バイトにリトルエンディアンの u64 で解除時刻を格納した想定
        if data.len() < 8 {
            return err!(ErrorCode::DataTooShort);
        }
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&data[0..8]);
        let current_ts = u64::from_le_bytes(buf);

        let new_ts = current_ts
            .checked_add(extra_secs)
            .ok_or(ErrorCode::TimeOverflow)?;
        // 新しいタイムスタンプを書き戻し
        data[0..8].copy_from_slice(&new_ts.to_le_bytes());

        msg!("Timelock extended to {}", new_ts);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExtendTimeLock<'info> {
    /// CHECK: owner フィールドの検証を行っていない生の AccountInfo
    #[account(mut)]
    pub timelock_account: AccountInfo<'info>,

    /// 呼び出し元が署名していることのみをチェック
    pub signer: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントのデータ長が 8 バイト未満です")]
    DataTooShort,
    #[msg("タイムスタンプのオーバーフローが発生しました")]
    TimeOverflow,
}
