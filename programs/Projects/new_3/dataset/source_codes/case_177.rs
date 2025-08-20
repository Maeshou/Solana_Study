use anchor_lang::prelude::*;
declare_id!("ExamVulnHasOne1111111111111111111111111");

/// 試験情報
#[account]
pub struct Exam {
    pub issuer:        Pubkey, // 試験発行者
    pub title:         String, // 試験名
    pub total_takers:  u64,    // 受験者数
}

/// 試験結果
#[account]
pub struct ExamResult {
    pub taker:         Pubkey, // 受験者
    pub exam:          Pubkey, // 本来は Exam.key() と一致すべき
    pub score:         u8,     // 得点
}

#[derive(Accounts)]
pub struct CreateExam<'info> {
    #[account(init, payer = issuer, space = 8 + 32 + 4 + 64 + 8)]
    pub exam:          Account<'info, Exam>,
    #[account(mut)]
    pub issuer:        Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitResult<'info> {
    /// Exam.issuer == issuer.key() は検証される
    #[account(mut, has_one = issuer)]
    pub exam:          Account<'info, Exam>,

    /// ExamResult.exam ⇔ exam.key() の検証がないため、
    /// 任意の結果アカウントを渡せてしまう
    #[account(init, payer = taker, space = 8 + 32 + 32 + 1)]
    pub result:        Account<'info, ExamResult>,

    #[account(mut)]
    pub taker:         Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FinalizeResult<'info> {
    /// Exam.issuer == issuer.key() は検証される
    #[account(mut, has_one = issuer)]
    pub exam:          Account<'info, Exam>,

    /// ExamResult.exam と exam.key() の検証がないため、
    /// 不正な結果アカウントを操作できる
    #[account(mut)]
    pub result:        Account<'info, ExamResult>,

    pub issuer:        Signer<'info>,
}

#[program]
pub mod exam_vuln {
    use super::*;

    /// 新規試験を作成
    pub fn create_exam(ctx: Context<CreateExam>, title: String) -> Result<()> {
        let e = &mut ctx.accounts.exam;
        e.issuer       = ctx.accounts.issuer.key();
        e.title        = title;
        e.total_takers = 0;
        Ok(())
    }

    /// 試験結果を提出（得点を記録）
    pub fn submit_result(ctx: Context<SubmitResult>, score: u8) -> Result<()> {
        let r = &mut ctx.accounts.result;
        let e = &ctx.accounts.exam;

        // 脆弱性ポイント:
        // r.exam = e.key(); と代入するのみで、
        // ExamResult.exam と Exam.key() の一致検証がない
        r.taker = ctx.accounts.taker.key();
        r.exam  = e.key();
        r.score = score;
        Ok(())
    }

    /// 試験結果を確定（受験者数を更新）
    pub fn finalize_result(ctx: Context<FinalizeResult>) -> Result<()> {
        let e  = &mut ctx.accounts.exam;
        let _r = &ctx.accounts.result;

        // 本来必須:
        // require_keys_eq!(ctx.accounts.result.exam, e.key(), ErrorCode::ExamMismatch);

        // 受験者数をインクリメント
        e.total_takers = e.total_takers.checked_add(1).unwrap();
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("ExamResult が指定の Exam と一致しません")]
    ExamMismatch,
}
