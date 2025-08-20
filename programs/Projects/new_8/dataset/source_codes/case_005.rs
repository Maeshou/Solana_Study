use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("Sc0reTabLe000000000000000000000000000001");

#[program]
pub mod score_table {
    use super::*;

    pub fn write_score(ctx: Context<WriteScore>, user_tag: [u8; 8], score: u32, bump: u8) -> Result<()> {
        // タグの整形と統計
        let mut tag = user_tag;
        let mut weight: u32 = 1;
        for i in 0..tag.len() {
            if tag[i].is_ascii_lowercase() { tag[i] = tag[i] - 32; }
            weight = weight.wrapping_mul(131).wrapping_add(tag[i] as u32);
        }

        // 入力 bump を使用（該当点）
        let seeds = [&ctx.accounts.admin.key().to_bytes()[..], &tag[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(ScoreError::BadCell))?;
        if addr != ctx.accounts.score_cell.key() {
            return Err(error!(ScoreError::BadCell));
        }

        // スコア更新：上限と係数
        let mut s = score;
        if s > 100000 { s = 100000; }
        let entry = &mut ctx.accounts.entry;
        entry.admin = ctx.accounts.admin.key();
        entry.tag = tag;
        entry.points = entry.points.saturating_add(s);
        entry.bias = entry.bias.wrapping_add(weight);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct WriteScore<'info> {
    #[account(mut)]
    pub entry: Account<'info, ScoreEntry>,
    /// CHECK: bump 正規化なし
    pub score_cell: AccountInfo<'info>,
    pub admin: AccountInfo<'info>,
}

#[account]
pub struct ScoreEntry {
    pub admin: Pubkey,
    pub tag: [u8; 8],
    pub points: u32,
    pub bias: u32,
}

#[error_code]
pub enum ScoreError {
    #[msg("Score cell PDA mismatch")]
    BadCell,
}
