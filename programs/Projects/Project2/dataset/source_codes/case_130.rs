use anchor_lang::prelude::*;

declare_id!("Vote111111111111111111111111111111111111");

#[program]
pub mod vote_manager {
    /// 新しい投票（Yes/No）を作成
    pub fn create_vote(
        ctx: Context<CreateVote>,
        question: String,
    ) -> Result<()> {
        // 質問長チェック
        if question.len() > 128 {
            return Err(ErrorCode::QuestionTooLong.into());
        }

        let vote = &mut ctx.accounts.vote;
        vote.owner    = ctx.accounts.creator.key();  // Signer Authorization
        vote.question = question;
        vote.yes      = 0;
        vote.no       = 0;
        vote.voters.clear();
        Ok(())
    }

    /// 投票を行う (true = Yes, false = No)
    pub fn cast_vote(
        ctx: Context<CastVote>,
        choice: bool,
    ) -> Result<()> {
        let vote = &mut ctx.accounts.vote;
        let user = ctx.accounts.voter.key();

        // アンサンブル：投票者リストに既に含まれるかチェック
        if vote.voters.iter().any(|&v| v == user) {
            return Err(ErrorCode::AlreadyVoted.into());
        }

        // Yes/No カウント
        if choice {
            vote.yes = vote.yes.checked_add(1).ok_or(ErrorCode::Overflow)?;
        } else {
            vote.no  = vote.no.checked_add(1).ok_or(ErrorCode::Overflow)?;
        }

        // 投票者登録
        vote.voters.push(user);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateVote<'info> {
    /// Reinit Attack 防止：同じアカウントを二度初期化できない
    #[account(init, payer = creator, space = 8 + 32 + 4 + 128 + 8 + 8 + 4 + (100 * 32))]
    pub vote:    Account<'info, VoteAccount>,

    /// 投票作成者（署名必須）
    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    /// 型チェック＆Owner Check
    #[account(mut)]
    pub vote:    Account<'info, VoteAccount>,

    /// 投票者（署名必須）
    pub voter:   Signer<'info>,
}

#[account]
pub struct VoteAccount {
    /// 投票を作成したユーザー
    pub owner:    Pubkey,
    /// 質問文（最大128文字）
    pub question: String,
    /// Yes 投票数
    pub yes:      u64,
    /// No 投票数
    pub no:       u64,
    /// 投票済みユーザーリスト（最大100人）
    pub voters:   Vec<Pubkey>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Question is too long")]
    QuestionTooLong,
    #[msg("Already voted")]
    AlreadyVoted,
    #[msg("Count overflow")]
    Overflow,
}
