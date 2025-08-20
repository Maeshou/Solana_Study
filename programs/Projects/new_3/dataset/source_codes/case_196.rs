use anchor_lang::prelude::*;
declare_id!("QnAVulnNoPop11111111111111111111111111111");

/// 質問情報
#[account]
pub struct Question {
    pub asker:      Pubkey,      // 質問者
    pub title:      String,      // 質問タイトル
    pub answerers:  Vec<Pubkey>, // 回答ユーザー一覧
}

/// 回答記録
#[account]
pub struct AnswerRecord {
    pub responder:  Pubkey,      // 回答者
    pub question:   Pubkey,      // 本来は Question.key() と一致すべき
    pub body:       String,      // 回答本文
}

#[derive(Accounts)]
pub struct CreateQuestion<'info> {
    #[account(init, payer = asker, space = 8 + 32 + 4 + 128 + 4 + (32 * 50))]
    pub question:   Account<'info, Question>,
    #[account(mut)]
    pub asker:      Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PostAnswer<'info> {
    /// Question.asker == asker.key() は不要ですが例示
    #[account(mut, has_one = asker)]
    pub question:   Account<'info, Question>,

    /// AnswerRecord.question ⇔ question.key() の検証がないため、
    /// 偽のレコードで任意の質問に回答可能
    #[account(init, payer = responder, space = 8 + 32 + 32 + 4 + 256)]
    pub record:     Account<'info, AnswerRecord>,

    #[account(mut)]
    pub asker:      Signer<'info>,
    #[account(mut)]
    pub responder:  Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RemoveAnswer<'info> {
    /// AnswerRecord.responder == responder.key() は検証される
    #[account(mut, has_one = responder)]
    pub record:     Account<'info, AnswerRecord>,

    /// question.key() ⇔ record.question のチェックがないため、
    /// 偽物のレコードで別の質問から回答を削除可能
    #[account(mut)]
    pub question:   Account<'info, Question>,

    pub responder:  Signer<'info>,
}

#[program]
pub mod qna_vuln_no_pop {
    use super::*;

    pub fn create_question(ctx: Context<CreateQuestion>, title: String) -> Result<()> {
        let q = &mut ctx.accounts.question;
        q.asker     = ctx.accounts.asker.key();
        q.title     = title;
        // answerers は init 時に空 Vec
        Ok(())
    }

    pub fn post_answer(ctx: Context<PostAnswer>, body: String) -> Result<()> {
        let q  = &mut ctx.accounts.question;
        let ar = &mut ctx.accounts.record;

        // 脆弱性ポイント:
        // ar.question = q.key(); の照合がない
        ar.responder = ctx.accounts.responder.key();
        ar.question  = q.key();
        ar.body      = body;

        // 回答ユーザー一覧に追加
        q.answerers.push(ar.responder);
        Ok(())
    }

    pub fn remove_answer(ctx: Context<RemoveAnswer>) -> Result<()> {
        let q = &mut ctx.accounts.question;

        // 本来必要:
        // require_keys_eq!(ctx.accounts.record.question, q.key(), ErrorCode::Mismatch);

        // Vec::truncate で最後に追加された回答者を除去（分岐・ループなし）
        let new_len = q.answerers.len().saturating_sub(1);
        q.answerers.truncate(new_len);
        Ok(())
    }
}
