use anchor_lang::prelude::*;
declare_id!("SurveyVuln1111111111111111111111111111111");

/// アンケート情報
#[account]
pub struct Survey {
    pub creator:    Pubkey,      // アンケート作成者
    pub title:      String,      // アンケートタイトル
    pub questions:  Vec<String>, // 質問リスト
}

/// 回答記録
#[account]
pub struct Response {
    pub respondent: Pubkey,      // 回答者
    pub survey:     Pubkey,      // 本来は Survey.key() と一致すべき
    pub answers:    Vec<String>, // 回答内容リスト
}

#[derive(Accounts)]
pub struct CreateSurvey<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 4 + 64 + 4 + (32 * 5))]
    pub survey:     Account<'info, Survey>,
    #[account(mut)]
    pub creator:    Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddQuestion<'info> {
    /// Survey.creator == creator.key() は検証される
    #[account(mut, has_one = creator)]
    pub survey:     Account<'info, Survey>,

    pub creator:    Signer<'info>,
}

#[derive(Accounts)]
pub struct SubmitResponse<'info> {
    /// Survey.creator == creator.key() は検証される（不要だが付与例）
    #[account(mut, has_one = creator)]
    pub survey:     Account<'info, Survey>,

    /// Response.survey ⇔ survey.key() の検証がないため、
    /// 任意の Response アカウントを渡しても通ってしまう
    #[account(init, payer = respondent, space = 8 + 32 + 32 + 4 + (32 * 5))]
    pub response:   Account<'info, Response>,

    #[account(mut)]
    pub respondent: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[program]
pub mod survey_vuln {
    use super::*;

    /// アンケートを作成
    pub fn create_survey(ctx: Context<CreateSurvey>, title: String) -> Result<()> {
        let s = &mut ctx.accounts.survey;
        s.creator   = ctx.accounts.creator.key();
        s.title     = title;
        // questions は init 時に空ベクタ
        Ok(())
    }

    /// 質問を追加
    pub fn add_question(ctx: Context<AddQuestion>, question: String) -> Result<()> {
        let s = &mut ctx.accounts.survey;
        // Vec::push で質問リストに追加
        s.questions.push(question);
        Ok(())
    }

    /// 回答を提出
    pub fn submit_response(ctx: Context<SubmitResponse>, answers: Vec<String>) -> Result<()> {
        let r = &mut ctx.accounts.response;
        let s = &ctx.accounts.survey;

        // 脆弱性ポイント:
        // r.survey = s.key(); の検証・制約がない
        r.respondent = ctx.accounts.respondent.key();
        r.survey     = s.key();
        r.answers    = answers;
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Response が指定の Survey と一致しません")]
    SurveyMismatch,
}
