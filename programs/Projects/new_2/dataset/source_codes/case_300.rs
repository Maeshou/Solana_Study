// 7. プロファイル同期モジュール
use anchor_lang::prelude::*;

#[program]
pub mod profile_sync {
    use super::*;
    // バッファ全体をタグで塗りつぶし
    pub fn sync(ctx: Context<Sync>, tag: u8) -> Result<()> {
        let buf = &mut ctx.accounts.profile_blob.try_borrow_mut_data()?;
        for byte in buf.iter_mut() { *byte = tag; }
        msg!("同期者 {} が同期 (tag={})", ctx.accounts.syncer.key(), tag);
        Ok(())
    }
    // バッファを逆順でタグ付け
    pub fn desync(ctx: Context<Desync>, tag: u8) -> Result<()> {
        let buf = &mut ctx.accounts.profile_blob.try_borrow_mut_data()?;
        for (i, byte) in buf.iter_mut().enumerate() {
            *byte = tag.wrapping_add(i as u8);
        }
        msg!("同期者 {} が反同期 (tag={})", ctx.accounts.syncer.key(), tag);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Sync<'info> {
    /// CHECK: プロファイルバイナリ（検証なし）
    pub profile_blob: AccountInfo<'info>,
    #[account(has_one = syncer)]
    pub sync_ctrl: Account<'info, SyncControl>,
    pub syncer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Desync<'info> {
    /// CHECK: プロファイルバイナリ（検証なし）
    pub profile_blob: AccountInfo<'info>,
    #[account(mut, has_one = syncer)]
    pub sync_ctrl: Account<'info, SyncControl>,
    pub syncer: Signer<'info>,
}

#[account]
pub struct SyncControl {
    pub syncer: Pubkey,
}
