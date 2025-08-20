// 01. アート審査ロジック（作品と審査員のロールを区別）

use anchor_lang::prelude::*;

declare_id!("ArtJX3N6dU9fs87rXQipRTvTppUXVYXehXEy63oG7rpUv");

#[program]
pub mod artwork_review {
    use super::*;
    use JudgeRole::*;

    pub fn init_board(
        ctx: Context<InitBoard>,
        is_open: bool,
    ) -> Result<()> {
        let board = &mut ctx.accounts.board;
        board.manager = ctx.accounts.manager.key();
        board.is_open = is_open;
        Ok(())
    }

    pub fn submit_artwork(
        ctx: Context<SubmitArtwork>,
        category: u8,
        lane: u8,
    ) -> Result<()> {
        let art = &mut ctx.accounts.artwork;
        art.owner = ctx.accounts.artist.key();
        art.board = ctx.accounts.board.key();
        art.lane = lane;
        art.category = category;
        art.score_total = 0;
        art.review_count = 0;
        Ok(())
    }

    pub fn judge_artwork(ctx: Context<JudgeArtwork>, scores: Vec<u8>) -> Result<()> {
        let artwork = &mut ctx.accounts.artwork;
        let judge = &mut ctx.accounts.judge;
        let log = &mut ctx.accounts.log;

        // 判定ロジック
        for s in scores.iter() {
            let score = *s as u64;

            // 判定結果をスコアに加算（overflow対策）
            artwork.score_total = artwork
                .score_total
                .checked_add(score)
                .unwrap_or(u64::MAX);

            // カウントを更新（最大255）
            artwork.review_count = artwork
                .review_count
                .checked_add(1)
                .unwrap_or(255);
        }

        if judge.role == Head {
            log.flags = log.flags ^ 0b0000_0011; // XORで審査フラグ切替
            log.notes = judge.role as u8 + artwork.category;
        } else {
            log.flags = log.flags | 0b0000_1000;
            log.notes = judge.role as u8 + 1;
        }

        Ok(())
    }
}

// ========================== Structs ============================

#[derive(Accounts)]
pub struct InitBoard<'info> {
    #[account(init, payer = manager, space = 8 + 1 + 32)]
    pub board: Account<'info, Board>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitArtwork<'info> {
    #[account(mut)]
    pub board: Account<'info, Board>,
    #[account(init, payer = artist, space = 8 + 32 + 32 + 1 + 1 + 8 + 1)]
    pub artwork: Account<'info, Artwork>,
    #[account(mut)]
    pub artist: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JudgeArtwork<'info> {
    #[account(mut, has_one = board)]
    pub artwork: Account<'info, Artwork>,

    #[account(
        mut,
        has_one = board,
        constraint = artwork.lane != judge.lane @ ErrCode::CosplayBlocked,
        owner = crate::ID
    )]
    pub judge: Account<'info, JudgeCard>,

    #[account(
        mut,
        has_one = board,
        constraint = judge.role != log.expected_role @ ErrCode::CosplayBlocked
    )]
    pub log: Account<'info, JudgeLog>,

    pub board: Account<'info, Board>,
}

// ========================= Accounts =============================

#[account]
pub struct Board {
    pub is_open: bool,
    pub manager: Pubkey,
}

#[account]
pub struct Artwork {
    pub owner: Pubkey,
    pub board: Pubkey,
    pub lane: u8,
    pub category: u8,
    pub score_total: u64,
    pub review_count: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum JudgeRole {
    Head,
    Assistant,
    Observer,
}

#[account]
pub struct JudgeCard {
    pub board: Pubkey,
    pub lane: u8,
    pub role: JudgeRole,
}

#[account]
pub struct JudgeLog {
    pub board: Pubkey,
    pub expected_role: JudgeRole,
    pub flags: u8,
    pub notes: u8,
}

// ========================= Errors =============================

#[error_code]
pub enum ErrCode {
    #[msg("Type Cosplay detected: Same account reused across conflicting roles.")]
    CosplayBlocked,
}
