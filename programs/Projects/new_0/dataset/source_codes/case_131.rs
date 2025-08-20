use anchor_lang::prelude::*;

declare_id!("Lead111111111111111111111111111111111111");

#[program]
pub mod leaderboard {
    /// プロフィールと初期スコアを作成
    pub fn create_score(
        ctx: Context<CreateScore>,
        initial_score: u32,
    ) -> Result<()> {
        let prof = &mut ctx.accounts.profile;
        prof.owner      = ctx.accounts.user.key();   // Signer Authorization
        prof.high_score = initial_score;
        Ok(())
    }

    /// 新しいスコアを提出（既存のハイスコアを超えた場合のみ更新）
    pub fn submit_score(
        ctx: Context<SubmitScore>,
        new_score: u32,
    ) -> Result<()> {
        let prof = &mut ctx.accounts.profile;
        let user = ctx.accounts.user.key();

        // 所有者チェック
        if prof.owner != user {
            return Err(ErrorCode::Unauthorized.into());
        }
        // スコアが更新条件を満たさない場合はエラー
        if new_score <= prof.high_score {
            return Err(ErrorCode::ScoreNotHighEnough.into());
        }
        prof.high_score = new_score;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateScore<'info> {
    /// Reinit Attack 防止：同一アカウントを二度初期化できない
    #[account(init, payer = user, space = 8 + 32 + 4)]
    pub profile:        Account<'info, ScoreAccount>,

    /// 操作するユーザー（署名必須）
    #[account(mut)]
    pub user:           Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitScore<'info> {
    /// 型チェック＆所有者チェック (Owner Check / Type Cosplay)
    #[account(mut)]
    pub profile:        Account<'info, ScoreAccount>,

    /// スコアを提出するユーザー（署名必須）
    pub user:           Signer<'info>,
}

#[account]
pub struct ScoreAccount {
    /// このプロフィールを操作できるユーザー
    pub owner:      Pubkey,
    /// ハイスコア
    pub high_score: u32,
}

#[error_code]
pub enum ErrorCode {
    #[msg("権限がありません")]
    Unauthorized,
    #[msg("新しいスコアが現在のハイスコアを超えていません")]
    ScoreNotHighEnough,
}
