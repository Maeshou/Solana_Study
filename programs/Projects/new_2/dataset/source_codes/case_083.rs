use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqSlotExt01");

#[program]
pub mod nft_slot_extension {
    use super::*;

    /// NFT用の追加装備スロットを付与する  
    /// (`slot_account` の owner チェックを一切行っていないため、  
    ///  攻撃者が他人のアカウントを指定して無制限スロットを付与できます)
    pub fn extend_slots(
        ctx: Context<ExtendSlots>,
        extra_slots: u8,  // 追加するスロット数
    ) -> Result<()> {
        let acct = &mut ctx.accounts.slot_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // ── データレイアウト想定 ──
        // [0]        : u8  現在のスロット数
        // [1]        : u8  最大許容スロット数（固定）
        // [2..]      : 拡張用

        if data.len() < 2 {
            return err!(ErrorCode::DataTooShort);
        }

        // 現在のスロット数取得＆更新（ownerチェックなし）
        let current = data[0];
        let max     = data[1];
        // saturating_add でオーバーフロー防止
        let new_count = current.saturating_add(extra_slots);
        // 制限を超えても制限値はチェックしない脆弱性
        data[0] = new_count;

        msg!(
            "Slots extended on {}: {} → {} (max {})",
            acct.key(),
            current,
            new_count,
            max
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExtendSlots<'info> {
    /// CHECK: owner == program_id の検証を全く行っていない AccountInfo
    #[account(mut)]
    pub slot_account: AccountInfo<'info>,

    /// 操作実行者の署名のみ検証
    pub operator: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("スロット管理アカウントのデータ領域が不足しています")]
    DataTooShort,
}
