use anchor_lang::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::to_vec;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqProfile01");

#[derive(Serialize, Deserialize)]
struct PlayerProfile {
    username:   String, // プレイヤー名
    avatar_uri: String, // アバタ ー画像URL
    updated_at: u64,    // 更新時刻（UNIX）
}

#[program]
pub mod nft_profile_update {
    use super::*;

    /// プレイヤーのプロフィールを更新する  
    /// (`profile_account` の owner チェックを一切行っていないため、  
    ///  攻撃者が他人のプロフィールアカウントを指定して  
    ///  任意の名前やアバターを設定できる脆弱性があります)
    pub fn update_profile(
        ctx: Context<UpdateProfile>,
        new_name: String,
        new_avatar: String,
    ) -> Result<()> {
        // 1) 構造体にまとめて JSON シリアライズ
        let now = Clock::get()?.unix_timestamp as u64;
        let profile = PlayerProfile {
            username:   new_name,
            avatar_uri: new_avatar,
            updated_at: now,
        };
        let json = to_vec(&profile).map_err(|_| ErrorCode::SerializationFailed)?;

        // 2) lamports ペナルティ：更新ごとに少額課金（任意のロジック）
        let fee: u64 = 10; // 例：10 lamports
        let payer = &mut ctx.accounts.user.to_account_info();
        let vault = &mut ctx.accounts.fee_vault.to_account_info();
        **payer.lamports.borrow_mut() = payer.lamports().saturating_sub(fee);
        **vault.lamports.borrow_mut() = vault.lamports().saturating_add(fee);

        // 3) アカウントに一括コピー
        let buf = &mut ctx.accounts.profile_account.data.borrow_mut();
        if buf.len() < json.len() {
            return err!(ErrorCode::AccountTooSmall);
        }
        buf[..json.len()].copy_from_slice(&json);

        msg!(
            "Profile {} updated by {} at {}",
            ctx.accounts.profile_account.key(),
            ctx.accounts.user.key(),
            now
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    /// CHECK: owner == program_id の検証をまったく行っていない AccountInfo
    #[account(mut)]
    pub profile_account: AccountInfo<'info>,

    /// 更新者（署名のみ検証）
    #[account(mut)]
    pub user:            Signer<'info>,

    /// 手数料を集めるアカウント（owner チェックなし）
    #[account(mut)]
    pub fee_vault:       AccountInfo<'info>,

    /// Clock Sysvar 用
    pub clock:           Sysvar<'info, Clock>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントのデータ領域が不足しています")]
    AccountTooSmall,
    #[msg("JSON シリアライズに失敗しました")]
    SerializationFailed,
}
