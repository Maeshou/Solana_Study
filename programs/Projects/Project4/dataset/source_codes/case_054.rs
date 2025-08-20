use anchor_lang::prelude::*;

declare_id!("SafeEx06XXXXXXX6666666666666666666666666666");

#[program]
pub mod example6 {
    use super::*;

    pub fn init_quiz(
        ctx: Context<InitQuiz>,
        total_questions: u8,
    ) -> Result<()> {
        let quiz = &mut ctx.accounts.quiz;
        quiz.total = total_questions;
        quiz.answered = 0;

        let score = &mut ctx.accounts.score;
        score.points = 0;

        let flag = &mut ctx.accounts.flag;
        flag.passed = false;
        Ok(())
    }

    pub fn answer_questions(
        ctx: Context<AnswerQuestions>,
        correct: u8,
    ) -> Result<()> {
        let quiz = &mut ctx.accounts.quiz;
        let answerable = (quiz.total - quiz.answered).min(correct);
        // ループで加点
        for _ in 0..answerable {
            ctx.accounts.score.points += 10;
            quiz.answered += 1;
        }
        // 分岐で合格判定
        ctx.accounts.flag.passed = ctx.accounts.score.points >= (quiz.total as u32 * 5);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitQuiz<'info> {
    #[account(init, payer = user, space = 8 + 1 + 1)]
    pub quiz: Account<'info, QuizData>,
    #[account(init, payer = user, space = 8 + 4)]
    pub score: Account<'info, ScoreData>,
    #[account(init, payer = user, space = 8 + 1)]
    pub flag: Account<'info, FlagData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AnswerQuestions<'info> {
    #[account(mut)] pub quiz: Account<'info, QuizData>,
    #[account(mut)] pub score: Account<'info, ScoreData>,
    #[account(mut)] pub flag: Account<'info, FlagData>,
}

#[account]
pub struct QuizData {
    pub total: u8,
    pub answered: u8,
}

#[account]
pub struct ScoreData {
    pub points: u32,
}

#[account]
pub struct FlagData {
    pub passed: bool,
}
