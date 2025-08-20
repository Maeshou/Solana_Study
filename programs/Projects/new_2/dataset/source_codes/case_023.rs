use anchor_lang::prelude::*;
use anchor_lang::sysvar::clock::Clock;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Cursor;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgqStakeV3abc");

#[program]
pub mod nft_stake_v3 {
    use super::*;

    /// NFTメタデータにステーク情報を複数フィールドで書き込む  
    /// （owner == program_id のチェックを行っていないため、任意のアカウントを操作可能）
    pub fn stake_nft(
        ctx: Context<StakeNft>,
        lock_duration: u64, // ロック秒数
    ) -> Result<()> {
        let acct_info = &mut ctx.accounts.nft_meta.to_account_info();
        let data = &mut acct_info.data.borrow_mut();

        // ── レイアウト想定 ──
        // [u8;1]   ステークフラグ
        // [u64;1]  ステーク開始タイムスタンプ
        // [Pubkey;1] ステーカーの Pubkey
        // [u64;1]  アンロック可能タイムスタンプ

        const EXPECTED_LEN: usize = 1 + 8 + 32 + 8;
        if data.len() < EXPECTED_LEN {
            return err!(ErrorCode::DataTooShort);
        }

        // 現在時刻とアンロック時刻を計算
        let now = Clock::get()?.unix_timestamp as u64;
        let unlock = now
            .checked_add(lock_duration)
            .ok_or(ErrorCode::TimestampOverflow)?;

        // Cursor + byteorder で直接バイト列を書き込む
        let mut cursor = Cursor::new(&mut data[..EXPECTED_LEN]);
        cursor.write_u8(1)?;                       // ステークフラグ = 1
        cursor.write_u64::<LittleEndian>(now)?;    // 開始時刻
        cursor.write_all(&ctx.accounts.user.key().to_bytes())?; // ステーカー Pubkey
        cursor.write_u64::<LittleEndian>(unlock)?; // アンロック時刻

        msg!(
            "Staked NFT {} by {} until {}",
            acct_info.key(),
            ctx.accounts.user.key(),
            unlock
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StakeNft<'info> {
    /// CHECK: ownerフィールドの確認を行っていない AccountInfo<'info>
    #[account(mut)]
    pub nft_meta: AccountInfo<'info>,

    /// 署名者であることのみ検証
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが想定より短いです")]
    DataTooShort,
    #[msg("タイムスタンプ計算でオーバーフローしました")]
    TimestampOverflow,
}
