// #4: Progression and Energy Management
// ドメイン: プレイヤーの進行度、レベル、エネルギーの管理。
// 安全対策: `PlayerProgression` と `PlayerEnergy` は親子関係を持ち、`has_one` で所有者が同一であることを強制。`PlayerProgression` 内の `role` フィールドで異なる役割を識別し、二重渡しを防ぐ。

declare_id!("A2B3C4D5E6F7G8H9I0J1K2L3M4N5O6P7Q8R9S0T1");

#[program]
pub mod player_progress {
    use super::*;

    pub fn initialize_player(ctx: Context<InitializePlayer>, player_role: u8) -> Result<()> {
        let progression = &mut ctx.accounts.player_progression;
        let energy = &mut ctx.accounts.player_energy;

        progression.owner = ctx.accounts.owner.key();
        progression.level = 1;
        progression.experience = 0;
        progression.role = player_role;

        energy.owner = ctx.accounts.owner.key();
        energy.current_energy = 100;
        energy.max_energy = 100;
        energy.last_updated = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn gain_experience_and_consume_energy(
        ctx: Context<GainExperienceAndConsumeEnergy>,
        xp_gain: u32,
        energy_cost: u32,
    ) -> Result<()> {
        let progression = &mut ctx.accounts.player_progression;
        let energy = &mut ctx.accounts.player_energy;

        // エネルギーを消費
        if energy.current_energy < energy_cost {
            return err!(ErrorCode::NotEnoughEnergy);
        }
        energy.current_energy = energy.current_energy.checked_sub(energy_cost).unwrap();

        // 経験値付与
        progression.experience = progression.experience.checked_add(xp_gain).unwrap();

        let mut xp_threshold = 100u32;
        let mut level_up = false;
        let new_level = progression.level.checked_add(1).unwrap();

        if progression.experience > xp_threshold {
            progression.experience = 0;
            progression.level = new_level;
            level_up = true;
        }

        if level_up {
            msg!("Player leveled up to {}!", new_level);
        } else {
            msg!("Player gained some experience.");
        }

        // ビット操作
        let mut status_flags: u8 = 0b00000001; // Active status
        if progression.level > 10 {
            status_flags |= 0b00000010; // Veteran status
        }
        msg!("Status flags: {:?}", status_flags);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePlayer<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 4 + 8 + 1 + 8,
        owner = crate::ID,
    )]
    pub player_progression: Account<'info, PlayerProgression>,
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 4 + 4 + 8,
        owner = crate::ID,
    )]
    pub player_energy: Account<'info, PlayerEnergy>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GainExperienceAndConsumeEnergy<'info> {
    #[account(
        mut,
        has_one = owner,
        constraint = player_progression.role != 0 @ ErrorCode::CosplayBlocked,
    )]
    pub player_progression: Account<'info, PlayerProgression>,
    #[account(
        mut,
        has_one = owner,
        // PlayerProgressionとPlayerEnergyが同一アカウントではないことを検証
        constraint = player_progression.key() != player_energy.key() @ ErrorCode::CosplayBlocked,
    )]
    pub player_energy: Account<'info, PlayerEnergy>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct PlayerProgression {
    pub owner: Pubkey,
    pub level: u32,
    pub experience: u64,
    pub role: u8,
    pub active_flags: u8,
}

#[account]
pub struct PlayerEnergy {
    pub owner: Pubkey,
    pub current_energy: u32,
    pub max_energy: u32,
    pub last_updated: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Account is being cosplayed as a different role.")]
    CosplayBlocked,
    #[msg("Not enough energy to perform this action.")]
    NotEnoughEnergy,
}
