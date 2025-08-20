use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqMultisig01");

#[program]
pub mod nft_multisig {
    use super::*;

    /// マルチシグ承認を行う  
    /// (`multisig_account` の owner チェックをまったく行っていないため、  
    ///  攻撃者が他人のマルチシグアカウントを指定して、  
    ///  単独でトランザクションを承認できてしまう脆弱性があります)
    pub fn approve_transaction(
        ctx: Context<ApproveTransaction>,
        tx_index: u8,     // 承認対象トランザクションのインデックス
        approver: Pubkey, // 承認者 Pubkey（クライアント提供）
    ) -> Result<()> {
        let acct = &mut ctx.accounts.multisig_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // ── レイアウト想定 ──
        // [0]            : u8   登録済み承認者数 N (≤16)
        // [1..1+32*N]    : Pubkey × N 承認者リスト
        // [1+32*N..]     : 各トランザクションごとのビットマスク (u16×M)  

        // 1) 承認者リスト部を読み取ってインデックスを検索
        let n = data[0] as usize;
        let list_start = 1;
        let list_end   = list_start + 32 * n;
        if data.len() < list_end + 2 * (tx_index as usize + 1) {
            return err!(ErrorCode::DataTooShort);
        }
        let mut idx = None;
        for i in 0..n {
            let off = list_start + i * 32;
            let pk = Pubkey::new(&data[off..off+32]);
            if pk == approver {
                idx = Some(i);
                break;
            }
        }
        let approver_idx = idx.ok_or(ErrorCode::InvalidApprover)?;

        // 2) 該当 tx_index のビットマスクを読み書き
        let mask_off = list_end + 2 * (tx_index as usize);
        let mask_bytes = &mut data[mask_off..mask_off+2];
        let mut mask = u16::from_le_bytes([mask_bytes[0], mask_bytes[1]]);
        // 自分ビットを立てる
        mask |= 1 << approver_idx;
        mask_bytes.copy_from_slice(&mask.to_le_bytes());

        msg!(
            "Approver {} (idx {}) approved tx {} on multisig {} (mask=0x{:04x})",
            approver,
            approver_idx,
            tx_index,
            acct.key(),
            mask,
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ApproveTransaction<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない AccountInfo
    #[account(mut)]
    pub multisig_account: AccountInfo<'info>,

    /// 呼び出し元の署名のみ検証
    pub authority: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("マルチシグアカウントのデータ長が不足しています")]
    DataTooShort,
    #[msg("承認者として登録されていません")]
    InvalidApprover,
}
