use anchor_lang::prelude::*;

declare_id!("ARENA222222222222222222222222222222222222222");

#[program]
pub mod arena_program {
    use super::*;
    /// 2人のキャラクターが互いに攻撃し、HPを更新する1ラウンド分の処理です。
    pub fn conduct_arena_duel_round(ctx: Context<ConductArenaDuel>) -> Result<()> {
        let player_one = &mut ctx.accounts.player_one;
        let player_two = &mut ctx.accounts.player_two;
        let duel_state = &mut ctx.accounts.duel_state;
        let clock = Clock::get()?;
        
        let luck_factor_one = (clock.slot % 10).saturating_add(1);
        let luck_factor_two = (clock.slot % 12).saturating_add(1);

        let damage_to_two = player_one.attack.saturating_mul(luck_factor_one).saturating_sub(player_two.defense);
        player_two.current_health = player_two.current_health.saturating_sub(damage_to_two);
        
        let damage_to_one = player_two.attack.saturating_mul(luck_factor_two).saturating_sub(player_one.defense);
        player_one.current_health = player_one.current_health.saturating_sub(damage_to_one);

        duel_state.round_number = duel_state.round_number.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConductArenaDuel<'info> {
    #[account(mut)]
    pub player_one: Account<'info, DuelCharacter>,
    #[account(mut)]
    pub player_two: Account<'info, DuelCharacter>,
    #[account(mut, has_one = player_one, has_one = player_two)]
    pub duel_state: Account<'info, DuelState>,
}

#[account]
pub struct DuelCharacter {
    pub owner: Pubkey,
    pub current_health: u64,
    pub attack: u64,
    pub defense: u64,
}

#[account]
pub struct DuelState {
    pub player_one: Pubkey,
    pub player_two: Pubkey,
    pub round_number: u32,
}