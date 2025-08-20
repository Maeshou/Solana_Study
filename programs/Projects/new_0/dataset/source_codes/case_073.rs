use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfRankingSys");

#[program]
pub mod user_ranking_system {
    use super::*;

    /// ユーザーがスコアを提出し、その中で最も高いスコアを `high_score` に保持します。
    /// 分岐を使わず、`.max()` メソッドで直接比較・更新します。
    pub fn submit_score(ctx: Context<SubmitScore>, new_score: u64) -> Result<()> {
        let record = &mut ctx.accounts.ranking_record;
        record.user = ctx.accounts.user.key();
        // 分岐なしで既存の high_score と new_score の大きい方を保持
        record.high_score = record.high_score.max(new_score);
        Ok(())
    }

    /// ユーザーの現在の最高スコアをログに出力します（分岐・ループなし）。
    pub fn view_score(ctx: Context<ViewScore>) -> Result<()> {
        let record = &ctx.accounts.ranking_record;
        msg!("User       : {:?}", record.user);
        msg!("High Score : {}", record.high_score);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitScore<'info> {
    /// PDA をシードで生成し、再初期化攻撃を防止。
    #[account(
        init,
        payer = user,
        space  = 8 + 32 + 8,
        seeds = [b"ranking", user.key().as_ref()],
        bump
    )]
    pub ranking_record: Account<'info, RankingRecord>,

    /// 提出操作に署名が必要
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ViewScore<'info> {
    /// PDA と所有者チェックで不正アクセスを防止
    #[account(
        seeds = [b"ranking", user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub ranking_record: Account<'info, RankingRecord>,

    pub user: Signer<'info>,
}

#[account]
pub struct RankingRecord {
    /// レコードオーナー
    pub user: Pubkey,
    /// 最高スコア
    pub high_score: u64,
}
