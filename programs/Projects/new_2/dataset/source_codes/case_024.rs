use anchor_lang::prelude::*;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Cursor;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgqStakeV5xyz");

#[program]
pub mod nft_stake_v5 {
    use super::*;

    /// NFTメタデータにステーク情報を書き込む  
    /// （owner == program_id のチェックを行っていないため、任意アカウントを操作可能）
    /// - `start_ts` と `unlock_ts` はクライアント側から渡され、そのまま信頼して使用している
    pub fn stake_nft(
        ctx: Context<StakeNft>,
        start_ts: u64,   // ステーク開始タイムスタンプ（クライアント提供）
        unlock_ts: u64,  // アンロック可能タイムスタンプ（クライアント提供）
    ) -> Result<()> {
        let acct_info = &mut ctx.accounts.nft_meta.to_account_info();
        let data = &mut acct_info.data.borrow_mut();

        // ── レイアウト想定 ──
        // [u8;1]    ステークフラグ
        // [u64;1]   ステーク開始タイムスタンプ
        // [Pubkey;1]ステーカーの Pubkey
        // [u64;1]   アンロックタイムスタンプ

        const REQUIRED_LEN: usize = 1 + 8 + 32 + 8;
        if data.len() < REQUIRED_LEN {
            return err!(ErrorCode::DataTooShort);
        }

        // Cursor + byteorder でまとめてバイト列を書き込む
        let mut cursor = Cursor::new(&mut data[..REQUIRED_LEN]);
        cursor.write_u8(1)?; // ステークフラグ = 1
        cursor.write_u64::<LittleEndian>(start_ts)?;    // クライアント提供の開始時刻
        cursor.write_all(&ctx.accounts.user.key().to_bytes())?; // ステーカー Pubkey
        cursor.write_u64::<LittleEndian>(unlock_ts)?;  // クライアント提供の解除時刻

        msg!(
            "Staked NFT {} by {} (start={}, unlock={})",
            acct_info.key(),
            ctx.accounts.user.key(),
            start_ts,
            unlock_ts
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StakeNft<'info> {
    /// CHECK: owner フィールドの検証を一切行っていない AccountInfo<'info>
    #[account(mut)]
    pub nft_meta: AccountInfo<'info>,

    /// 呼び出し元が署名者であることのみ検証
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが想定より短いです")]
    DataTooShort,
}
