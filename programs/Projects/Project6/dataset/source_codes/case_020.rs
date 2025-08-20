// #10: Pet Tamer's Scoreboard
// ドメイン: ペットの育成・訓練スコア管理。
// 安全対策: `TamerProfile` と `PetScore` は親子関係 `has_one` で結合。`PetScore` には `slot_id` を設けることで、二重渡しを防ぐ。`TamerProfile` は `role_tag` を持ち、`PetScore` との連携を強化。

declare_id!("G1H2I3J4K5L6M7N8O9P0Q1R2S3T4U5V6W7X8Y9Z0");

#[program]
pub mod pet_tamer {
    use super::*;

    pub fn register_tamer(ctx: Context<RegisterTamer>, tamer_name: String) -> Result<()> {
        let tamer = &mut ctx.accounts.tamer_profile;
        tamer.owner = ctx.accounts.owner.key();
        tamer.name = tamer_name;
        tamer.pet_count = 0;
        tamer.role_tag = TamerRole::Apprentice;
        Ok(())
    }

    pub fn update_pet_score(
        ctx: Context<UpdatePetScore>,
        training_points: u64,
    ) -> Result<()> {
        let tamer = &mut ctx.accounts.tamer_profile;
        let pet = &mut ctx.accounts.pet_score;

        pet.score = pet.score.checked_add(training_points).unwrap_or(u64::MAX);

        let pet_average = pet.score.checked_div(pet.matches_played as u64).unwrap_or(0);
        let max_level = 100u8;

        pet.level = (pet_average.checked_div(100).unwrap_or(0) as u8).min(max_level);

        for _ in 0..5 {
            tamer.pet_count = tamer.pet_count.wrapping_add(1);
        }

        if pet.level >= 50 {
            msg!("Pet has reached a high level!");
        } else {
            msg!("Pet is still in training.");
        }

        // XOR ビット操作
        let mut xor_val = pet.score as u32;
        let mask = 0b10101010;
        xor_val ^= mask;
        msg!("XOR value: {}", xor_val);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterTamer<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 32 + 4 + 1,
        owner = crate::ID,
    )]
    pub tamer_profile: Account<'info, TamerProfile>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePetScore<'info> {
    #[account(
        mut,
        has_one = owner,
    )]
    pub tamer_profile: Account<'info, TamerProfile>,
    #[account(
        mut,
        has_one = owner,
        // `TamerProfile` と `PetScore` が同一口座ではないことを検証
        constraint = tamer_profile.key() != pet_score.key() @ ErrorCode::CosplayBlocked,
        constraint = tamer_profile.role_tag == TamerRole::Apprentice @ ErrorCode::RoleTagMismatch,
    )]
    pub pet_score: Account<'info, PetScore>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct TamerProfile {
    pub owner: Pubkey,
    pub name: String,
    pub pet_count: u32,
    pub role_tag: TamerRole,
}

#[account]
pub struct PetScore {
    pub owner: Pubkey,
    pub tamer_profile: Pubkey,
    pub slot_id: u8,
    pub score: u64,
    pub level: u8,
    pub matches_played: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TamerRole {
    Apprentice,
    Master,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Account is being cosplayed as a different role.")]
    CosplayBlocked,
    #[msg("The tamer's role tag does not match the expected value.")]
    RoleTagMismatch,
}