use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_arena_game {
    use super::*;

    pub fn record_match_result(ctx: Context<RecordMatchResult>, winner_score: u32, loser_score: u32) -> Result<()> {
        let match_record = &mut ctx.accounts.match_record;

        // スコアの検証
        if winner_score <= loser_score {
            return err!(ArenaError::InvalidScore);
        }

        // 勝利者のEloレーティングを更新するなどの処理
        let mut elo_change = 10;
        if winner_score > loser_score * 2 {
            elo_change = 20; // 圧勝ボーナス
        }

        match_record.player_one = *ctx.accounts.player_one.key;
        match_record.player_two = *ctx.accounts.player_two.key;
        match_record.winner = *ctx.accounts.winner.key;
        match_record.elo_change = elo_change;
        match_record.bump = *ctx.bumps.get("match_record").unwrap();

        // 試合内容をログに出力するダミーループ
        let mut round = 0;
        loop {
            round += 1;
            msg!("Simulating battle log for round {}", round);
            if round > 5 {
                break;
            }
        }
        
        msg!("Match recorded. Winner: {}. Elo change: {}", match_record.winner, match_record.elo_change);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RecordMatchResult<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 32 + 32 + 4 + 1,
        // player_oneとplayer_twoのアドレスをソートしてseedに使い、順序に依らず一意のPDAを生成する
        seeds = [
            b"match".as_ref(), 
            // 決定論的にするためにキーをハッシュ化して使う
            &hash(&[player_one.key().to_bytes(), player_two.key().to_bytes()].concat()).to_bytes()
        ],
        bump
    )]
    pub match_record: Account<'info, MatchRecord>,
    /// CHECK: This is not dangerous because we are just checking the key
    pub player_one: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we are just checking the key
    pub player_two: AccountInfo<'info>,
    /// CHECK: Winner must be one of the players
    pub winner: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MatchRecord {
    pub player_one: Pubkey,
    pub player_two: Pubkey,
    pub winner: Pubkey,
    pub elo_change: u32,
    pub bump: u8,
}

#[error_code]
pub enum ArenaError {
    #[msg("Winner's score must be greater than loser's score.")]
    InvalidScore,
}