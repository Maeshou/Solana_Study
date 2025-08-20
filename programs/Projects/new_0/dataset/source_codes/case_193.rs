use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct ReviewScheduler(pub u8, pub Vec<(Vec<u8>, i64)>); // (bump, Vec<(doc_hash, due_ts)>)

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzVE");

#[error_code]
pub enum ErrorCode {
    #[msg("Maximum number of documents scheduled")]
    MaxDocumentsReached,
    #[msg("Document not found")]
    DocumentNotFound,
}

#[program]
pub mod review_scheduler {
    use super::*;

    const MAX_DOCS: usize = 10;

    /// 初期化：内部 Vec は空、bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let b = *ctx.bumps.get("scheduler").unwrap();
        ctx.accounts.scheduler.0 = b;
        Ok(())
    }

    /// レビュー予定登録：件数制限チェック＋(doc_hash, due_ts) 追加
    pub fn schedule_review(
        ctx: Context<Modify>,
        doc_hash: Vec<u8>,
        due_ts: i64,
    ) -> Result<()> {
        let list = &mut ctx.accounts.scheduler.1;
        if list.len() >= MAX_DOCS {
            return err!(ErrorCode::MaxDocumentsReached);
        }
        list.push((doc_hash, due_ts));
        Ok(())
    }

    /// レビュー完了マーク：該当ドキュメントを一括除去
    pub fn mark_reviewed(
        ctx: Context<Modify>,
        doc_hash: Vec<u8>,
    ) -> Result<()> {
        let list = &mut ctx.accounts.scheduler.1;
        list.retain(|(hash, _)| {
            if *hash == doc_hash {
                false
            } else {
                true
            }
        });
        Ok(())
    }

    /// 期限超過分を一括削除
    pub fn purge_past(
        ctx: Context<Modify>,
    ) -> Result<()> {
        let now  = ctx.accounts.clock.unix_timestamp;
        let list = &mut ctx.accounts.scheduler.1;
        list.retain(|&(_, due)| {
            if due >= now {
                true
            } else {
                false
            }
        });
        Ok(())
    }

    /// 今後のレビュー数をログ出力
    pub fn count_upcoming(
        ctx: Context<Modify>,
    ) -> Result<()> {
        let now  = ctx.accounts.clock.unix_timestamp;
        let mut cnt = 0u64;
        for &(_, due) in ctx.accounts.scheduler.1.iter() {
            if due > now {
                cnt = cnt.wrapping_add(1);
            }
        }
        msg!("Upcoming reviews: {}", cnt);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"scheduler", authority.key().as_ref()],
        bump,
        // discriminator(8)+bump(1)+Vec len(4)+max10*(4+32+8)
        space = 8 + 1 + 4 + 10 * (4 + 32 + 8)
    )]
    pub scheduler: Account<'info, ReviewScheduler>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub clock:     Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"scheduler", authority.key().as_ref()],
        bump = scheduler.0,
    )]
    pub scheduler: Account<'info, ReviewScheduler>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub clock:     Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}
