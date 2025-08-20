use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqDeactivateV2");

#[program]
pub mod nft_listing_deactivation_v2 {
    use super::*;

    /// マーケットプレイス出品を一時停止／再開する  
    /// （`listing_account` の owner チェックを行っていないため、
    ///  攻撃者が任意のアカウントを指定して、
    ///  他人の出品を停止または再開させてしまう脆弱性があります）
    pub fn toggle_listing_active(
        ctx: Context<ToggleListingActive>,
    ) -> Result<()> {
        let acct = &mut ctx.accounts.listing_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // ── 先頭バイトを first_mut() + XOR で反転 ──
        if let Some(flag_byte) = data.first_mut() {
            // 0b0000_0000 ↔ 0b0000_0001 を切り替え
            *flag_byte ^= 1;
        } else {
            return err!(ErrorCode::DataTooShort);
        }

        // ── オプション: トグル時刻を末尾に記録 ──
        let now_slot = Clock::get()?.slot;
        let slot_bytes = now_slot.to_le_bytes();
        if data.len() >= 1 + slot_bytes.len() {
            let tail = &mut data[data.len() - slot_bytes.len()..];
            tail.copy_from_slice(&slot_bytes);
        }

        msg!(
            "Listing {} active flag toggled, new flag={} (updated at slot {})",
            acct.key(),
            data.first().unwrap(),
            now_slot
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ToggleListingActive<'info> {
    /// CHECK: owner == program_id の検証を行っていない生の AccountInfo
    #[account(mut)]
    pub listing_account: AccountInfo<'info>,

    /// 操作実行者が署名していることのみ検証
    pub operator: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが想定より短いです")]
    DataTooShort,
}
