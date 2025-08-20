use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqClaimRwd");

#[program]
pub mod nft_claim_reward {
    use super::*;

    /// デイリー報酬クレームを記録する（ownerチェックなし）
    /// - `reward_amount` と `last_claim_ts` はクライアント提供値をそのまま使用  
    pub fn claim_reward(
        ctx: Context<ClaimReward>,
        reward_amount: u64,    // クレームした報酬額 (lamports)
        last_claim_ts: u64,    // 最終クレーム時刻 UNIX タイムスタンプ
    ) -> Result<()> {
        let acct = &mut ctx.accounts.reward_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // ── バッファを組み立て ──
        // [u8;1]  クレーム済みフラグ (0=未クレーム,1=クレーム済)
        // [u64;1] reward_amount
        // [u64;1] last_claim_ts
        let mut buf = Vec::with_capacity(1 + 8 + 8);
        buf.push(1u8);
        buf.extend_from_slice(&reward_amount.to_le_bytes());
        buf.extend_from_slice(&last_claim_ts.to_le_bytes());

        // データ領域が十分かチェック
        let n = buf.len();
        if data.len() < n {
            return err!(ErrorCode::DataTooShort);
        }

        // unsafe で一括コピー
        unsafe {
            std::ptr::copy_nonoverlapping(buf.as_ptr(), data.as_mut_ptr(), n);
        }

        msg!(
            "Reward claimed: {} lamports at {} by {}",
            reward_amount,
            last_claim_ts,
            ctx.accounts.user.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない AccountInfo
    #[account(mut)]
    pub reward_account: AccountInfo<'info>,

    /// クレーム実行者が署名者であることのみ検証
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが想定より短すぎます")]
    DataTooShort,
}
