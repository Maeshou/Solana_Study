use anchor_lang::prelude::*;

declare_id!("STAKE333333333333333333333333333333333333333");

#[program]
pub mod staking_program {
    use super::*;
    /// NFTキャラクターをステーキングプールに預ける
    pub fn stake_character(ctx: Context<StakeCharacter>) -> Result<()> {
        let character = &mut ctx.accounts.player_character;
        let stake_pool = &mut ctx.accounts.stake_pool;
        let clock = Clock::get()?;

        character.is_staked = true;
        character.staked_at_timestamp = clock.unix_timestamp;
        stake_pool.total_staked_characters = stake_pool.total_staked_characters.saturating_add(1);
        
        msg!("Character staked successfully.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StakeCharacter<'info> {
    #[account(mut, has_one = owner, constraint = !player_character.is_staked)]
    pub player_character: Account<'info, PlayerCharacter>,
    #[account(mut, constraint = stake_pool.total_staked_characters < 10000 @ GameErrorCode::StakingPoolFull)]
    pub stake_pool: Account<'info, StakePool>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct PlayerCharacter {
    pub owner: Pubkey,
    pub is_staked: bool,
    pub staked_at_timestamp: i64,
}

#[account]
pub struct StakePool {
    pub total_staked_characters: u64,
}

#[error_code]
pub enum GameErrorCode {
    #[msg("Staking pool is currently full.")]
    StakingPoolFull,
}