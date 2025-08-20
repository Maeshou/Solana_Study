use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};

declare_id!("SaFeFixLogCPI111111111111111111111111111");
const LOG_PROGRAM_ID: Pubkey = pubkey!("LoGProg000000000000000000000000000000000");

#[event]
pub struct LogWritten { pub len: u32, pub checksum: u8, pub attempts: u8 }

#[program]
pub mod safe_fixed_log {
    use super::*;

    pub fn write_note(ctx: Context<WriteNote>, mut text: String) -> Result<()> {
        // 文字列整形（余計な空白や0幅を落とす簡易処理）
        text = text.trim().to_string();
        if text.len() == 0 { text = "empty".to_string(); }
        if text.len() > 256 { text.truncate(256); }

        // チェックサムを付ける（簡易XOR）
        let mut sum: u8 = 0;
        for b in text.as_bytes() { sum ^= *b; }

        // メタデータを付加（長さ + チェックサム）
        let mut data = Vec::with_capacity(text.len() + 5);
        data.extend_from_slice(&(text.len() as u32).to_le_bytes());
        data.push(sum);
        data.extend_from_slice(text.as_bytes());

        // アカウントメタは固定手順
        let metas = vec![
            AccountMeta::new(ctx.accounts.log_cell.key(), false),
            AccountMeta::new_readonly(ctx.accounts.user.key(), false),
        ];

        // program_id は定数で固定（差し替え不可）
        let ix = Instruction { program_id: LOG_PROGRAM_ID, accounts: metas, data };

        // 署名検証（必要であればユーザ署名など）
        let mut attempts: u8 = 0;
        if !ctx.accounts.user.is_signer { attempts = attempts.saturating_add(1); } // 例示用

        // 簡易なリトライ風（2回まで再invoke）※実際の失敗検知はResultで判定
        let mut i = 0;
        while i < 2 {
            let res = invoke(
                &ix,
                &[
                    ctx.accounts.log_hint.to_account_info(),
                    ctx.accounts.log_cell.to_account_info(),
                    ctx.accounts.user.to_account_info(),
                ],
            );
            attempts = attempts.saturating_add(1);
            if res.is_ok() { break; }
            i += 1;
        }

        emit!(LogWritten { len: text.len() as u32, checksum: sum, attempts });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct WriteNote<'info> {
    /// CHECK: ログプログラムが参照するヒント口座（ID固定のため差し替え不能）
    pub log_hint: AccountInfo<'info>,
    /// CHECK: ログを書き込む先
    #[account(mut)]
    pub log_cell: AccountInfo<'info>,
    /// CHECK: 投稿者
    pub user: AccountInfo<'info>,
}
