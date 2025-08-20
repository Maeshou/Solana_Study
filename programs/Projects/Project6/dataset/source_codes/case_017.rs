// #7: Scoring and Ranking System
// ドメイン: プレイヤーのスコアリングとランキング。
// 安全対策: `Scoreboard` と `PlayerScore` は親子関係 `has_one` で結合。`PlayerScore` 内に `slot_id` を設けることで、同一の `Scoreboard` 内で複数の `PlayerScore` を操作する際に、異なる `slot_id` の二重渡しを防止。

declare_id!("R8S9T0U1V2W3X4Y5Z6A7B8C9D0E1F2G3H4I5J6K7");

#[program]
pub mod scoring_system {
    use super::*;

    pub fn initialize_scoreboard(ctx: Context<InitializeScoreboard>, title: String) -> Result<()> {
        let scoreboard = &mut ctx.accounts.scoreboard;
        scoreboard.title = title;
        scoreboard.entry_count = 0;
        Ok(())
    }

    pub fn update_player_score(
        ctx: Context<UpdatePlayerScore>,
        slot_id_player: u8,
        new_score: u64,
        slot_id_opponent: u8,
    ) -> Result<()> {
        let player_score = &mut ctx.accounts.player_score;
        let opponent_score = &mut ctx.accounts.opponent_score;
        let scoreboard = &mut ctx.accounts.scoreboard;

        player_score.score = new_score;
        opponent_score.score = opponent_score.score.checked_add(new_score / 10).unwrap_or(u64::MAX);

        player_score.matches_played = player_score.matches_played.checked_add(1).unwrap();
        opponent_score.matches_played = opponent_score.matches_played.checked_add(1).unwrap();

        let mut average_score = (player_score.score / (player_score.matches_played as u64)).min(1000);
        
        if average_score > 500 {
            msg!("Player is performing well!");
        } else {
            msg!("Player needs more practice.");
        }

        // ビットマスクでフラグを操作
        player_score.status_flags |= 0b00000001; // Active status

        if new_score > 1000 {
            player_score.status_flags |= 0b00000010; // High Score Achiever
        } else {
            player_score.status_flags &= 0b11111101; // Remove High Score Achiever
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeScoreboard<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 4,
        owner = crate::ID,
    )]
    pub scoreboard: Account<'info, Scoreboard>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(slot_id_player: u8, new_score: u64, slot_id_opponent: u8)]
pub struct UpdatePlayerScore<'info> {
    #[account(mut)]
    pub scoreboard: Account<'info, Scoreboard>,
    #[account(
        mut,
        has_one = scoreboard,
        // `player_score` と `opponent_score` が同一口座ではないことを確認
        constraint = player_score.key() != opponent_score.key() @ ErrorCode::CosplayBlocked,
        // `slot_id` の不一致を要件化
        constraint = player_score.slot_id != opponent_score.slot_id @ ErrorCode::CosplayBlocked,
    )]
    pub player_score: Account<'info, PlayerScore>,
    #[account(
        mut,
        has_one = scoreboard,
        constraint = player_score.key() != opponent_score.key() @ ErrorCode::CosplayBlocked,
        constraint = opponent_score.slot_id == slot_id_opponent @ ErrorCode::SlotIdMismatch,
    )]
    pub opponent_score: Account<'info, PlayerScore>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Scoreboard {
    pub title: String,
    pub entry_count: u32,
}

#[account]
pub struct PlayerScore {
    pub owner: Pubkey,
    pub scoreboard: Pubkey,
    pub slot_id: u8,
    pub score: u64,
    pub matches_played: u32,
    pub status_flags: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Account is being cosplayed as a different role.")]
    CosplayBlocked,
    #[msg("Provided slot ID does not match the account's slot ID.")]
    SlotIdMismatch,
}