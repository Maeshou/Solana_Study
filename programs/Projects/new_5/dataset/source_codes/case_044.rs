// 5. Game Progress & Energy Management
declare_id!("M2N5P9Q3R7S1T4U8V2W6X0Y4Z8A2B6C0D4E7F0");

use anchor_lang::prelude::*;

#[program]
pub mod player_progress {
    use super::*;

    pub fn init_player(ctx: Context<InitPlayer>, initial_level: u32, initial_xp: u64) -> Result<()> {
        let player = &mut ctx.accounts.player;
        player.owner = ctx.accounts.owner.key();
        player.level = initial_level;
        player.xp = initial_xp;
        player.energy = 100;
        player.max_energy = 100;
        player.is_active = true;
        msg!("Player {} initialized with level {}", player.owner, player.level);
        Ok(())
    }

    pub fn init_session(ctx: Context<InitSession>, session_id: u64) -> Result<()> {
        let session = &mut ctx.accounts.session;
        session.player = ctx.accounts.player.key();
        session.session_id = session_id;
        session.xp_gained = 0;
        session.energy_consumed = 0;
        session.is_finished = false;
        msg!("New session {} started for player {}", session.session_id, session.player);
        Ok(())
    }

    pub fn progress_in_session(ctx: Context<ProgressInSession>, actions: Vec<u8>) -> Result<()> {
        let player = &mut ctx.accounts.player;
        let session = &mut ctx.accounts.session;

        if session.is_finished {
            return Err(ErrorCode::SessionFinished.into());
        }

        let mut total_xp_gained: u64 = 0;
        let mut total_energy_cost: u8 = 0;
        let max_actions = 20;

        for &action_type in actions.iter().take(max_actions) {
            let energy_cost = if action_type == 1 { 5 } else { 10 };
            
            if player.energy < energy_cost {
                player.is_active = false; // Player is now 'inactive' until they regain energy
                session.is_finished = true;
                msg!("Not enough energy. Session terminated.");
                break;
            } else {
                let xp_gain = if action_type == 1 { 20 } else { 50 };
                player.xp = player.xp.saturating_add(xp_gain as u64);
                player.energy = player.energy.saturating_sub(energy_cost);
                total_xp_gained += xp_gain as u64;
                total_energy_cost += energy_cost;

                // Check for level up condition
                let xp_to_next_level = player.level * 1000;
                if player.xp >= xp_to_next_level as u64 {
                    player.level = player.level.saturating_add(1);
                    player.xp = player.xp.saturating_sub(xp_to_next_level as u64);
                    player.max_energy = player.max_energy.saturating_add(10);
                    msg!("Player leveled up to {}!", player.level);
                }
            }
        }

        msg!("Session {} processed. Gained {} XP, consumed {} energy.", session.session_id, total_xp_gained, total_energy_cost);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPlayer<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 8 + 1 + 1 + 1)]
    pub player: Account<'info, PlayerStats>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitSession<'info> {
    #[account(mut, has_one = player)]
    pub player: Account<'info, PlayerStats>,
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 1 + 1)]
    pub session: Account<'info, GameSession>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProgressInSession<'info> {
    #[account(mut)]
    pub player: Account<'info, PlayerStats>,
    #[account(mut, has_one = player)]
    pub session: Account<'info, GameSession>,
}

#[account]
pub struct PlayerStats {
    pub owner: Pubkey,
    pub level: u32,
    pub xp: u64,
    pub energy: u8,
    pub max_energy: u8,
    pub is_active: bool,
}

#[account]
pub struct GameSession {
    pub player: Pubkey,
    pub session_id: u64,
    pub xp_gained: u64,
    pub energy_consumed: u8,
    pub is_finished: bool,
}

#[error_code]
pub enum ProgressError {
    #[msg("The game session has already finished.")]
    SessionFinished,
}