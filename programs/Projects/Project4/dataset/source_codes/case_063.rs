use anchor_lang::prelude::*;

declare_id!("SafeEx07Leaderboard11111111111111111111111");

#[program]
pub mod example7 {
    use super::*;

    /// リーダーボードを初期化
    pub fn init_leaderboard(
        ctx: Context<InitBoard>,
        s1: u32,
        s2: u32,
        s3: u32,
    ) -> Result<()> {
        let board = &mut ctx.accounts.board;
        // スコアを降順にソートしてセット
        let mut arr = [s1, s2, s3];
        // バブルソート
        for i in 0..3 {
            for j in 0..2 {
                if arr[j] < arr[j+1] {
                    let tmp = arr[j];
                    arr[j]  = arr[j+1];
                    arr[j+1]= tmp;
                }
            }
        }
        board.top1 = arr[0];
        board.top2 = arr[1];
        board.top3 = arr[2];
        Ok(())
    }

    /// 新スコアを投稿し、必要に応じて更新
    pub fn submit_score(
        ctx: Context<SubmitScore>,
        score: u32,
    ) -> Result<()> {
        let b = &mut ctx.accounts.board;
        // スコア比較と入れ替えを繰り返し
        if score > b.top3 {
            b.top3 = score;
            if b.top3 > b.top2 {
                let tmp = b.top2; b.top2 = b.top3; b.top3 = tmp;
                if b.top2 > b.top1 {
                    let tmp2 = b.top1; b.top1 = b.top2; b.top2 = tmp2;
                }
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBoard<'info> {
    #[account(init, payer = auth, space = 8 + 4*3)]
    pub board: Account<'info, Leaderboard>,
    #[account(mut)] pub auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitScore<'info> {
    #[account(mut)] pub board: Account<'info, Leaderboard>,
}

#[account]
pub struct Leaderboard {
    pub top1: u32,
    pub top2: u32,
    pub top3: u32,
}
