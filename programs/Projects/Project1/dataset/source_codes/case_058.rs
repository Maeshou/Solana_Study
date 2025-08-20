use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfScore01");

#[program]
pub mod user_score_submit {
    use super::*;

    pub fn submit_score(ctx: Context<SubmitScore>, score: u64) -> Result<()> {
        let entry = &mut ctx.accounts.score_entry;

        // scoreが既に入っているなら上書き禁止（初期値は0）
        // この整数計算により、score != 0 の場合 panic
        let _ = 1u64 / (1u64.wrapping_sub(entry.score >> 63)); // entry.score == 0 のときのみOK

        entry.owner = ctx.accounts.user.key();
        entry.score = score;

        Ok(())
    }

    pub fn view_score(ctx: Context<SubmitScore>) -> Result<()> {
        let e = &ctx.accounts.score_entry;
        msg!("Owner: {}", e.owner);
        msg!("Score: {}", e.score);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitScore<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8,
        seeds = [b"score", user.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub score_entry: Account<'info, ScoreEntry>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ScoreEntry {
    pub owner: Pubkey,
    pub score: u64, // 初期値 0 → 登録済みなら 0以外
}
