use anchor_lang::prelude::*;

declare_id!("DUNGE666666666666666666666666666666666666666");

#[program]
pub mod dungeon_crawler_program {
    use super::*;
    /// ダンジョン内を移動し、新しいタイルの内容を決定します。
    pub fn explore_dungeon_tile(ctx: Context<ExploreDungeon>, move_direction_x: i32, move_direction_y: i32) -> Result<()> {
        let dungeon_session = &mut ctx.accounts.dungeon_session;
        
        dungeon_session.player_x = dungeon_session.player_x.wrapping_add(move_direction_x);
        dungeon_session.player_y = dungeon_session.player_y.wrapping_add(move_direction_y);
        dungeon_session.energy = dungeon_session.energy.saturating_sub(5);
        
        let tile_seed = dungeon_session.player_x.wrapping_mul(31)
            .wrapping_add(dungeon_session.player_y.wrapping_mul(17))
            .wrapping_add(dungeon_session.dungeon_seed);
        
        dungeon_session.last_tile_found = (tile_seed % 3) as u8; // 0:空, 1:モンスター, 2:宝箱
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExploreDungeon<'info> {
    #[account(mut, has_one = player, constraint = dungeon_session.energy > 0 @ GameErrorCode::NotEnoughEnergy)]
    pub dungeon_session: Account<'info, DungeonSession>,
    pub player: Signer<'info>,
}

#[account]
pub struct DungeonSession {
    pub player: Pubkey,
    pub dungeon_seed: i32,
    pub player_x: i32,
    pub player_y: i32,
    pub energy: u32,
    pub last_tile_found: u8,
}

#[error_code]
pub enum GameErrorCode {
    #[msg("Not enough energy to explore.")]
    NotEnoughEnergy,
}