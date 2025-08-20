use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUL");

#[program]
pub mod feedback_system {
    use super::*;

    /// フィードバック管理アカウント初期化  
    pub fn initialize_feedback(
        ctx: Context<InitializeFeedback>,
    ) -> Result<()> {
        let feedback = &mut ctx.accounts.feedback;
        // ゼロクリア済み → owner, bump, last_feedback_ts のみ設定
        feedback.owner            = ctx.accounts.user.key();
        feedback.bump             = *ctx.bumps.get("feedback").unwrap();
        feedback.last_feedback_ts = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }

    /// フィードバック投稿：評価を加算し、平均評価とタイムスタンプを更新  
    pub fn submit_feedback(
        ctx: Context<ModifyFeedback>,
        rating: u8,
    ) -> Result<()> {
        let f = &mut ctx.accounts.feedback;
        f.feedback_count   = f.feedback_count.wrapping_add(1);
        f.total_rating     = f.total_rating.wrapping_add(rating as u64);
        f.average_rating   = (f.total_rating / f.feedback_count) as u8;
        f.last_feedback_ts = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }

    /// フィードバックリセット：カウント・合計・平均をクリアし、時刻を更新  
    pub fn reset_feedback(
        ctx: Context<ModifyFeedback>,
    ) -> Result<()> {
        let f = &mut ctx.accounts.feedback;
        f.feedback_count    = 0;
        f.total_rating      = 0;
        f.average_rating    = 0;
        f.last_feedback_ts  = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }

    /// 所有権移転：新しいオーナーを設定し、時刻を更新  
    pub fn transfer_ownership(
        ctx: Context<ModifyFeedback>,
        new_owner: Pubkey,
    ) -> Result<()> {
        let f = &mut ctx.accounts.feedback;
        f.owner            = new_owner;
        f.last_feedback_ts = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8, feedback_id: u64)]
pub struct InitializeFeedback<'info> {
    /// ゼロで初期化後、必要なフィールドだけ設定する
    #[account(
        init_zeroed,
        payer = user,
        seeds = [b"feedback", user.key().as_ref(), &feedback_id.to_le_bytes()],
        bump,
        space = 8 + 32 + 1 + 8 + 8 + 1 + 8  // discriminator + owner + bump + feedback_count + total_rating + average_rating + last_feedback_ts
    )]
    pub feedback: Account<'info, Feedback>,

    /// プログラム利用者（初期オーナー）
    #[account(mut)]
    pub user: Signer<'info>,

    /// 時刻取得用
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyFeedback<'info> {
    /// 既存の Feedback（PDA 検証 + has_one = owner）
    #[account(
        mut,
        seeds = [b"feedback", owner.key().as_ref(), &feedback.feedback_count.to_le_bytes()],
        bump = feedback.bump,
        has_one = owner
    )]
    pub feedback: Account<'info, Feedback>,

    /// Feedback 所有者（署名必須）
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    /// 時刻取得用
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Feedback {
    pub owner:            Pubkey,  // アカウント所有者
    pub bump:             u8,      // PDA 用バンプ
    pub feedback_count:   u64,     // 投稿数
    pub total_rating:     u64,     // 評価合計
    pub average_rating:   u8,      // 平均評価
    pub last_feedback_ts: i64,     // 最終フィードバック時刻
}
