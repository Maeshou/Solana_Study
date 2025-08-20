use anchor_lang::prelude::*;
use std::convert::TryInto;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqNonceV1xyz");

#[program]
pub mod payment_with_nonce {
    use super::*;

    /// 支払いを実行しつつ「Nonce 管理アカウント」を更新する  
    /// （`nonce_account` の owner チェックを一切行っていないため、
    ///  攻撃者が他人の Nonce アカウントをリセットして
    ///  再請求や二重請求を行えます）
    pub fn pay_with_nonce(
        ctx: Context<PayWithNonce>,
        payment_amount: u64,
        submitted_nonce: u64,  // クライアントから渡される一意の連番
    ) -> Result<()> {
        // 1) Lamports の移動
        let sender = &mut ctx.accounts.payer.to_account_info();
        let receiver = &mut ctx.accounts.recipient.to_account_info();
        **sender.lamports.borrow_mut() = sender
            .lamports()
            .checked_sub(payment_amount)
            .ok_or(ErrorCode::InsufficientFunds)?;
        **receiver.lamports.borrow_mut() += payment_amount;

        // 2) Nonce の検証＆更新（owner チェックなし）
        let acct = &mut ctx.accounts.nonce_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // ── バッファレイアウト想定 ──
        // [0..8]   u64: last_nonce
        if data.len() < 8 {
            return err!(ErrorCode::DataTooShort);
        }

        // split_at_mut で先頭 8 バイトを nonce_slice として取り出す
        let (nonce_slice, _) = data.split_at_mut(8);
        let last_nonce = u64::from_le_bytes(nonce_slice.try_into().unwrap());

        // クライアントから渡された submitted_nonce は必ず last_nonce より大きくなければならない
        if submitted_nonce <= last_nonce {
            return err!(ErrorCode::InvalidNonce);
        }

        // nonce_slice を新しい submitted_nonce で上書き
        nonce_slice.copy_from_slice(&submitted_nonce.to_le_bytes());

        msg!(
            "Payment {} lamports processed with nonce {} → {}",
            payment_amount,
            last_nonce,
            submitted_nonce
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PayWithNonce<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない生の AccountInfo
    #[account(mut)]
    pub nonce_account: AccountInfo<'info>,

    /// 支払い元アカウント（署名のみ検証）
    #[account(mut)]
    pub payer: Signer<'info>,

    /// 支払い先アカウント（owner チェックなし）
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが想定より短いです")]
    DataTooShort,
    #[msg("十分な残高がありません")]
    InsufficientFunds,
    #[msg("Nonce が不正です（古いか重複しています）")]
    InvalidNonce,
}
