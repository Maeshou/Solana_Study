use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqRentalNoSlice");

#[program]
pub mod nft_rental_cancel_noslice {
    use super::*;

    /// NFTレンタル契約をキャンセルし、アイテムを返却する  
    /// (`rental_account` の owner チェックを一切行っていないため、  
    ///  攻撃者が他人のレンタル契約を指定して勝手にキャンセル＆返却を行えます)  
    pub fn cancel_rental(
        ctx: Context<CancelRental>,
        current_time: u64,   // クライアント提供の現在時刻
    ) -> Result<()> {
        let mut data = ctx.accounts.rental_account.data.borrow_mut();

        // 必要なヘッダ長を分割で取得
        // [lessee_pubkey (32)] [expires (8)] [...]
        let (head, tail) = data.split_at_mut(32);
        let (expires_bytes, _rest) = tail.split_at_mut(8);

        // lessee Pubkey を head から生成
        let lessee = Pubkey::new(head);

        // expiry を expires_bytes から生成
        let expiry = expires_bytes
            .try_into()
            .map(u64::from_le_bytes)
            .map_err(|_| ErrorCode::DataTooShort)?;

        // 有効期限チェック
        if current_time < expiry {
            return err!(ErrorCode::NotYetExpired);
        }

        // 返却処理（CPI 等は省略）
        // …

        // データ全体をゼロクリア
        data.fill(0);

        msg!(
            "Rental {} canceled at {} (lessee was {})",
            ctx.accounts.rental_account.key(),
            current_time,
            lessee
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CancelRental<'info> {
    /// CHECK: owner == program_id の検証を行っていない
    #[account(mut)]
    pub rental_account: AccountInfo<'info>,

    /// キャンセルを実行するユーザー（署名のみ検証）
    pub operator:       Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("レンタル契約アカウントのデータが不足しています")]
    DataTooShort,
    #[msg("まだレンタル期間が終了していません")]
    NotYetExpired,
}
