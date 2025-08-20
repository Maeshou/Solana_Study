use anchor_lang::prelude::*;
declare_id!("JobAppVuln111111111111111111111111111111");

/// 求人情報
#[account]
pub struct JobPosting {
    pub organizer:    Pubkey,        // 求人作成者
    pub title:        String,        // 求人タイトル
    pub applicants:   Vec<Pubkey>,   // 応募者一覧
}

/// 応募記録
#[account]
pub struct ApplicationRecord {
    pub candidate:    Pubkey,        // 応募者
    pub posting:      Pubkey,        // 本来は JobPosting.key() と一致すべき
    pub resume:       String,        // 履歴書テキスト
}

#[derive(Accounts)]
pub struct CreatePosting<'info> {
    #[account(init, payer = organizer, space = 8 + 32 + 4 + 64 + 4 + (32 * 20))]
    pub posting:      Account<'info, JobPosting>,
    #[account(mut)]
    pub organizer:    Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitApplication<'info> {
    /// JobPosting.organizer == organizer.key() は検証される
    #[account(mut, has_one = organizer)]
    pub posting:      Account<'info, JobPosting>,

    /// ApplicationRecord.posting ⇔ posting.key() の検証がない
    #[account(init, payer = candidate, space = 8 + 32 + 32 + 4 + 256)]
    pub record:       Account<'info, ApplicationRecord>,

    #[account(mut)]
    pub organizer:    Signer<'info>,
    #[account(mut)]
    pub candidate:    Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawApplications<'info> {
    /// JobPosting.organizer == organizer.key() は検証される
    #[account(mut, has_one = organizer)]
    pub posting:      Account<'info, JobPosting>,

    /// ApplicationRecord.candidate == candidate.key() は検証される
    #[account(mut, has_one = candidate)]
    pub record:       Account<'info, ApplicationRecord>,

    pub organizer:    Signer<'info>,
    pub candidate:    Signer<'info>,
}

#[program]
pub mod job_app_vuln {
    use super::*;

    /// 求人を作成
    pub fn create_posting(ctx: Context<CreatePosting>, title: String) -> Result<()> {
        let jp = &mut ctx.accounts.posting;
        jp.organizer  = ctx.accounts.organizer.key();
        jp.title      = title;
        // applicants は init 時に空ベクタ
        Ok(())
    }

    /// 応募を提出
    pub fn submit_application(ctx: Context<SubmitApplication>, resume: String) -> Result<()> {
        let jp = &mut ctx.accounts.posting;
        let ar = &mut ctx.accounts.record;

        // 脆弱性ポイント:
        // ar.posting = jp.key(); の検証・制約がない
        ar.candidate = ctx.accounts.candidate.key();
        ar.posting   = jp.key();
        ar.resume    = resume;

        // Vec::push で応募者一覧に追加
        jp.applicants.push(ar.candidate);
        Ok(())
    }

    /// 応募を取り下げ（一覧から除外）
    pub fn withdraw_applications(ctx: Context<WithdrawApplications>) -> Result<()> {
        let jp = &mut ctx.accounts.posting;
        let ar = &ctx.accounts.record;

        // 本来は必須:
        // require_keys_eq!(ar.posting, jp.key(), ErrorCode::PostingMismatch);

        // Vec::retain で該当候補者を一覧から除去
        jp.applicants.retain(|&pk| pk != ar.candidate);
        Ok(())
    }
}
