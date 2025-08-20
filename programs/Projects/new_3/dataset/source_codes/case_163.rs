use anchor_lang::prelude::*;
declare_id!("FitChal1111111111111111111111111111111111");

/// チャレンジ情報
#[account]
pub struct Challenge {
    pub creator:     Pubkey, // チャレンジ作成者
    pub reward_pool: u64,    // 報酬プール残高
}

/// 参加記録
#[account]
pub struct ChallengeRecord {
    pub participant: Pubkey, // 参加者
    pub challenge:   Pubkey, // 本来は Challenge.key() と一致すべき
    pub completed:   bool,   // 完了フラグ
    pub rewarded:    bool,   // 報酬付与済みフラグ
}

#[derive(Accounts)]
pub struct CreateChallenge<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 8)]
    pub challenge:    Account<'info, Challenge>,
    #[account(mut)]
    pub creator:      Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinChallenge<'info> {
    /// Challenge.creator == creator.key() は不要
    #[account(mut)]
    pub challenge:    Account<'info, Challenge>,

    #[account(init, payer = participant, space = 8 + 32 + 32 + 1 + 1)]
    pub record:       Account<'info, ChallengeRecord>,

    #[account(mut)]
    pub participant:  Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompleteChallenge<'info> {
    /// ChallengeRecord.participant == participant.key() は検証される
    #[account(mut, has_one = participant)]
    pub record:       Account<'info, ChallengeRecord>,

    /// Challenge.key() と record.challenge の検証がないため、
    /// 別のチャレンジのレコードを渡されても通ってしまう
    #[account(mut)]
    pub challenge:    Account<'info, Challenge>,

    pub participant:  Signer<'info>,
}

#[program]
pub mod fitness_challenge_vuln {
    use super::*;

    /// チャレンジを作成
    pub fn create_challenge(ctx: Context<CreateChallenge>, pool: u64) -> Result<()> {
        let c = &mut ctx.accounts.challenge;
        c.creator      = ctx.accounts.creator.key();
        c.reward_pool  = pool;
        Ok(())
    }

    /// チャレンジに参加（完了前レコード作成）
    pub fn join_challenge(ctx: Context<JoinChallenge>) -> Result<()> {
        let rec = &mut ctx.accounts.record;
        // rec.challenge = ctx.accounts.challenge.key(); と代入するだけ
        rec.participant = ctx.accounts.participant.key();
        rec.challenge   = ctx.accounts.challenge.key();
        rec.completed   = false;
        rec.rewarded    = false;
        Ok(())
    }

    /// 完了＆報酬付与
    pub fn complete(ctx: Context<CompleteChallenge>, reward: u64) -> Result<()> {
        let rec = &mut ctx.accounts.record;
        let ch  = &mut ctx.accounts.challenge;

        // 本来は必須：
        // require_keys_eq!(rec.challenge, ch.key(), ErrorCode::ChallengeMismatch);

        // 脆弱性：チャレンジの一致検証がないため、
        // 任意のレコードで報酬プールを減らし、報酬済みにマークできる
        rec.completed = true;
        if !rec.rewarded {
            ch.reward_pool = ch.reward_pool.saturating_sub(reward);
            rec.rewarded   = true;
        }
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("ChallengeRecord が指定の Challenge と一致しません")]
    ChallengeMismatch,
}
