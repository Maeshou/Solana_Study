use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfRankingSysV");

#[program]
pub mod user_ranking_system_insecure {
    use super::*;

    /// ユーザーがスコアを提出し、その中で最も高いスコアを `high_score` に保持します。
    /// 署名チェックは一切行われず、`.max()` メソッドで直接比較・更新します。
    pub fn submit_score(ctx: Context<SubmitScore>, new_score: u64) {
        let record = &mut ctx.accounts.ranking_record;
        record.user = *ctx.accounts.user.key;
        record.high_score = record.high_score.max(new_score);
    }

    /// ユーザーの現在の最高スコアをログに出力します（分岐・ループなし）。
    /// 署名チェックは行われません。
    pub fn view_score(ctx: Context<ViewScore>) {
        let record = &ctx.accounts.ranking_record;
        msg!("User       : {:?}", record.user);
        msg!("High Score : {}", record.high_score);
    }
}

#[derive(Accounts)]
pub struct SubmitScore<'info> {
    /// PDA をシードで生成し、再初期化攻撃を防止。
    #[account(
        init_if_needed,
        payer  = user,
        space  = 8 + 32 + 8,
        seeds  = [b"ranking", user.key().as_ref()],
        bump
    )]
    pub ranking_record: Account<'info, RankingRecord>,

    /// ユーザーアカウント（署名チェック omitted intentionally）
    pub user:            AccountInfo<'info>,

    pub system_program:  Program<'info, System>,
}

#[derive(Accounts)]
pub struct ViewScore<'info> {
    /// 既存のランキングデータ PDA
    #[account(
        seeds = [b"ranking", user.key().as_ref()],
        bump
    )]
    pub ranking_record: Account<'info, RankingRecord>,

    /// ユーザーアカウント（署名チェック omitted intentionally）
    pub user:            AccountInfo<'info>,
}

#[account]
pub struct RankingRecord {
    /// レコードオーナー
    pub user:       Pubkey,
    /// 最高スコア
    pub high_score: u64,
}
