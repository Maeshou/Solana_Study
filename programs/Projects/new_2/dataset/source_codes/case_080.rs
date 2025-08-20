use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqInsrClaim1");

#[program]
pub mod nft_insurance_claim {
    use super::*;

    /// NFT保険金を請求する  
    /// （`insurance_account` の owner チェックを一切行っていないため、  
    ///  攻撃者が他人のNFTを指定して無制限に保険金を請求できます）
    pub fn claim_insurance(ctx: Context<ClaimInsurance>) -> Result<()> {
        let ins_acc = &mut ctx.accounts.insurance_account.to_account_info();
        let pool    = &mut ctx.accounts.insurance_pool.to_account_info();
        let user    = &mut ctx.accounts.user.to_account_info();

        // ── データレイアウト想定 ──
        // [0..8]   u64  保険金上限（max_claim）
        // [8..16]  u64  最終請求時 UNIXタイムスタンプ（last_claim_ts）
        let data = &mut ins_acc.data.borrow_mut();
        if data.len() < 16 {
            return err!(ErrorCode::DataTooShort);
        }
        let max_claim    = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let last_ts      = u64::from_le_bytes(data[8..16].try_into().unwrap());

        // 冷却期間 1 日 (= 86400 秒)
        let now = Clock::get()?.unix_timestamp as u64;
        if now < last_ts.saturating_add(86400) {
            return err!(ErrorCode::TooSoon);
        }

        // 1) 保険金をプールからユーザーに送金
        let amount = max_claim;
        **pool.lamports.borrow_mut() = pool
            .lamports()
            .checked_sub(amount)
            .ok_or(ErrorCode::InsufficientPool)?;
        **user.lamports.borrow_mut() = user
            .lamports()
            .checked_add(amount)
            .unwrap();

        // 2) 最終請求時刻を更新 (再利用防止)
        data[8..16].copy_from_slice(&now.to_le_bytes());

        msg!(
            "Insurance claimed {} lamports from pool {} by {}",
            amount,
            pool.key(),
            user.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimInsurance<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない AccountInfo  
    #[account(mut)]
    pub insurance_account: AccountInfo<'info>,

    /// 保険金プールアカウント（owner チェックなし）  
    #[account(mut)]
    pub insurance_pool:    AccountInfo<'info>,

    /// 保険を請求するユーザー（署名のみ検証）  
    #[account(mut)]
    pub user:              Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが不足しています")]
    DataTooShort,
    #[msg("まだクールダウン中です")]
    TooSoon,
    #[msg("プールに十分な資金がありません")]
    InsufficientPool,
}
