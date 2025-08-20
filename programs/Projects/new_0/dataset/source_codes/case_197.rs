use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct LanguageProgress(pub u8, pub Vec<(Vec<u8>, u64)>); // (bump, Vec<(language_code, total_secs)>)

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzVI");

#[error_code]
pub enum ErrorCode {
    #[msg("Maximum number of languages tracked")]
    MaxLanguagesReached,
    #[msg("Language not found")]
    LanguageNotFound,
}

#[program]
pub mod language_progress {
    use super::*;

    const MAX_LANGUAGES: usize = 6;

    /// 初期化：内部 Vec は空、bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let b = *ctx.bumps.get("progress").unwrap();
        ctx.accounts.progress.0 = b;
        Ok(())
    }

    /// 言語追加：件数制限チェック＋初期 0 で追加
    pub fn add_language(ctx: Context<Modify>, code: Vec<u8>) -> Result<()> {
        let list = &mut ctx.accounts.progress.1;
        if list.len() >= MAX_LANGUAGES {
            return err!(ErrorCode::MaxLanguagesReached);
        }
        list.push((code, 0));
        Ok(())
    }

    /// 学習時間記録：該当言語を検索し、時間を加算
    pub fn log_time(ctx: Context<Modify>, code: Vec<u8>, secs: u64) -> Result<()> {
        let list = &mut ctx.accounts.progress.1;
        let mut found = false;
        for entry in list.iter_mut() {
            if entry.0 == code {
                entry.1 = entry.1.wrapping_add(secs);
                found = true;
            }
        }
        if found == false {
            return err!(ErrorCode::LanguageNotFound);
        }
        Ok(())
    }

    /// 言語削除：該当コードを一括除去
    pub fn remove_language(ctx: Context<Modify>, code: Vec<u8>) -> Result<()> {
        let list = &mut ctx.accounts.progress.1;
        list.retain(|(c, _)| {
            if *c == code {
                false
            } else {
                true
            }
        });
        Ok(())
    }

    /// 合計学習時間をログ出力
    pub fn total_time(ctx: Context<Modify>) -> Result<()> {
        let list = &ctx.accounts.progress.1;
        let mut sum = 0u64;
        for &(_, secs) in list.iter() {
            sum = sum.wrapping_add(secs);
        }
        msg!("Total learning time: {} seconds", sum);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = user,
        seeds = [b"progress", user.key().as_ref()],
        bump,
        // discriminator(8)+bump(1)+Vec len(4)+max6*(4+5+8)
        // language_code: 4-byte length + up to 5-byte code
        space = 8 + 1 + 4 + 6 * (4 + 5 + 8)
    )]
    pub progress: Account<'info, LanguageProgress>,
    #[account(mut)]
    pub user:      Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"progress", user.key().as_ref()],
        bump = progress.0,
    )]
    pub progress: Account<'info, LanguageProgress>,
    #[account(signer)]
    pub user:      AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
