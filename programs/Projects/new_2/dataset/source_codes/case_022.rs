use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgqFeatTgl01");

#[program]
pub mod insecure_feature_toggle {
    use super::*;

    /// feature_account のビットマスクから指定機能のフラグを反転する
    /// （Owner Check をまったく行っていないため、任意のアカウントで通ってしまう）
    pub fn toggle_feature(ctx: Context<ToggleFeature>, feature_index: u8) -> Result<()> {
        let acct = &mut ctx.accounts.feature_account.to_account_info();
        let mut data = acct.data.borrow_mut();

        // ★ owner==program_id の検証をしていない！
        // データ先頭 4 バイトを機能フラグ用ビットマスク (u32 little endian) とみなす
        if data.len() < 4 {
            return err!(ErrorCode::DataTooSmall);
        }

        // 現在のマスクを読み出し
        let mut mask_bytes = [0u8; 4];
        mask_bytes.copy_from_slice(&data[0..4]);
        let mut mask = u32::from_le_bytes(mask_bytes);

        // feature_index 番目のビットを反転（0～31 の範囲内で安全に）
        let idx = (feature_index.min(31)) as u32;
        mask ^= 1 << idx;

        // マスクを書き戻し
        data[0..4].copy_from_slice(&mask.to_le_bytes());

        msg!("Toggled feature {}: new mask = {:#034b}", feature_index, mask);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ToggleFeature<'info> {
    /// CHECK: owner フィールドの検証を一切行っていない生の AccountInfo
    #[account(mut)]
    pub feature_account: AccountInfo<'info>,

    /// 呼び出し元が署名していることのみ検証
    pub signer: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("データ領域が 4 バイト未満です")]
    DataTooSmall,
}
