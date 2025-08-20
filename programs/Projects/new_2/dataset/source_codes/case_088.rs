use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqReferral01");

#[program]
pub mod nft_referral_reward {
    use super::*;

    /// 紹介者に報酬を付与し、紹介数を更新する  
    /// (`referral_account` の owner チェックを一切行っていないため、  
    ///  攻撃者が他人の紹介アカウントを指定して  
    ///  報酬を無制限に受け取れます)
    pub fn claim_referral(
        ctx: Context<ClaimReferral>,
        reward_amount: u64,    // 付与する報酬 (lamports)
    ) -> Result<()> {
        let acc = &mut ctx.accounts.referral_account.to_account_info();
        let data = &mut acc.data.borrow_mut();

        // ―― 想定レイアウト ――
        // [0..8]   u64: 紹介者に支払われた累計報酬
        // [8..16]  u64: 紹介数カウント
        if data.len() < 16 {
            return err!(ErrorCode::DataTooShort);
        }

        // 1) 既存の累計報酬と紹介数を読み出し
        let mut paid_buf = [0u8; 8];
        paid_buf.copy_from_slice(&data[0..8]);
        let mut count_buf = [0u8; 8];
        count_buf.copy_from_slice(&data[8..16]);
        let paid_before  = u64::from_le_bytes(paid_buf);
        let count_before = u64::from_le_bytes(count_buf);

        // 2) 紹介数インクリメント＆報酬累計更新
        let paid_after  = paid_before.saturating_add(reward_amount);
        let count_after = count_before.saturating_add(1);
        data[0..8].copy_from_slice(&paid_after.to_le_bytes());
        data[8..16].copy_from_slice(&count_after.to_le_bytes());

        // 3) 報酬プールから紹介者に支払う
        **ctx.accounts.reward_pool.to_account_info().lamports.borrow_mut() = 
            ctx.accounts.reward_pool.to_account_info().lamports()
            .saturating_sub(reward_amount);
        **ctx.accounts.referrer.to_account_info().lamports.borrow_mut() += reward_amount;

        msg!(
            "Referral claimed: referrer={} count {}→{}, paid {}→{} lamports",
            ctx.accounts.referrer.key(),
            count_before,
            count_after,
            paid_before,
            paid_after
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimReferral<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない AccountInfo
    #[account(mut)]
    pub referral_account: AccountInfo<'info>,

    /// 報酬プールアカウント（owner チェックなし）
    #[account(mut)]
    pub reward_pool:      AccountInfo<'info>,

    /// 紹介者の受取先（署名のみ検証）
    #[account(mut)]
    pub referrer:         Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが想定より短いです")]
    DataTooShort,
}
