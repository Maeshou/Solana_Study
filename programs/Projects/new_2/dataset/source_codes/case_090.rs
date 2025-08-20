use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqStamina02");

#[program]
pub mod nft_stamina_recovery_v2 {
    use super::*;

    pub fn recover_stamina(ctx: Context<RecoverStamina>, amount: u16) -> Result<()> {
        let acc = &mut ctx.accounts.stamina_account.to_account_info();
        let buf = &mut acc.data.borrow_mut();

        if buf.len() < 12 {
            return err!(ErrorCode::DataTooShort);
        }

        // 1) スタミナ読み出し（u16 リトルエンディアン）
        let cur = u16::from_le_bytes([buf[0], buf[1]]);
        let max = u16::from_le_bytes([buf[2], buf[3]]);

        // 2) スタミナ回復処理（最大値を超えないように制限）
        let recovered = cur.saturating_add(amount).min(max);
        let recovered_bytes = recovered.to_le_bytes();
        buf[0] = recovered_bytes[0];
        buf[1] = recovered_bytes[1];

        // 3) 現在時刻を書き込む（最後の8バイト）
        let now_ts = Clock::get()?.unix_timestamp;
        let now_bytes = now_ts.to_le_bytes();
        let start = buf.len() - 8;
        for i in 0..8 {
            buf[start + i] = now_bytes[i];
        }

        msg!(
            "Stamina on {}: recovered {} → {} (capped at max {})",
            acc.key(),
            cur,
            recovered,
            max
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RecoverStamina<'info> {
    /// CHECK: 検証なしの任意アカウント（脆弱性あり）
    #[account(mut)]
    pub stamina_account: AccountInfo<'info>,

    /// 実行者（署名のみ確認）
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("スタミナアカウントのデータ長が不足しています")]
    DataTooShort,
}
