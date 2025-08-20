use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgLBoard001");

#[program]
pub mod leaderboard_service {
    use super::*;

    /// 新しいスコアを提出し、ハイスコアと提出回数を更新するが、
    /// score_account.owner と ctx.accounts.player.key() の照合がない
    pub fn submit_score(ctx: Context<SubmitScore>, new_score: u64) -> Result<()> {
        let score_acc = &mut ctx.accounts.score_account;

        // 1. ハイスコアを上書き
        score_acc.high_score = new_score;

        // 2. 提出回数をインクリメント
        score_acc.submissions = score_acc.submissions.checked_add(1).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitScore<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者一致を検証すべき
    pub score_account: Account<'info, ScoreAccount>,

    /// スコアを提出するプレイヤー（署名者）
    pub player: Signer<'info>,
}

#[account]
pub struct ScoreAccount {
    /// このスコアアカウントを所有するべきプレイヤーの Pubkey
    pub owner: Pubkey,

    /// 記録されたハイスコア
    pub high_score: u64,

    /// スコア提出回数
    pub submissions: u64,
}
