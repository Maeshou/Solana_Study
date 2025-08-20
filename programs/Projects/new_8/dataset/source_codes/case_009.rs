use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("QuEsTTrAcK00000000000000000000000000005");

#[program]
pub mod quest_track {
    use super::*;

    pub fn push_step(ctx: Context<PushStep>, chapter: u16, notes: Vec<u8>, bump: u8) -> Result<()> {
        // ノート整形と章インデックス調整
        let mut n = notes.clone();
        if n.len() > 80 { n.truncate(80); }
        let mut chapter_id = chapter;
        if chapter_id < 1 { chapter_id = 1; }

        // 入力 bump をそのまま利用（該当点）
        let seeds = [&ctx.accounts.player.key().to_bytes()[..], &chapter_id.to_le_bytes()[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(QuestErr::Mismatch))?;
        if addr != ctx.accounts.route_cell.key() {
            return Err(error!(QuestErr::Mismatch));
        }

        // 記録の反映
        let q = &mut ctx.accounts.route;
        q.player = ctx.accounts.player.key();
        q.chapter = chapter_id;
        q.memo = n;
        q.steps = q.steps.saturating_add(1);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct PushStep<'info> {
    #[account(mut)]
    pub route: Account<'info, Route>,
    /// CHECK:
    pub route_cell: AccountInfo<'info>,
    pub player: AccountInfo<'info>,
}

#[account]
pub struct Route {
    pub player: Pubkey,
    pub chapter: u16,
    pub memo: Vec<u8>,
    pub steps: u32,
}

#[error_code]
pub enum QuestErr {
    #[msg("Route cell PDA mismatch")]
    Mismatch,
}
