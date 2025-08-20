use anchor_lang::prelude::*;

declare_id!("PETTR777777777777777777777777777777777777777");

#[program]
pub mod pet_training_program {
    use super::*;
    /// ペットの適性に基づいてスキルの経験値を上昇させ、クールダウンを設定します。
    pub fn train_pet_skill(ctx: Context<TrainPet>) -> Result<()> {
        let pet = &mut ctx.accounts.pet;
        let clock = Clock::get()?;

        let base_exp_gain = 50;
        let aptitude_bonus = pet.aptitude.saturating_mul(5);
        pet.agility_exp = pet.agility_exp.saturating_add(base_exp_gain + aptitude_bonus);

        pet.last_trained_timestamp = clock.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TrainPet<'info> {
    #[account(mut, has_one = owner, constraint = Clock::get().unwrap().unix_timestamp > pet.last_trained_timestamp + 3600 @ GameErrorCode::PetOnCooldown)]
    pub pet: Account<'info, Pet>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Pet {
    pub owner: Pubkey,
    pub agility_exp: u64,
    pub aptitude: u32,
    pub last_trained_timestamp: i64,
}

#[error_code]
pub enum GameErrorCode {
    #[msg("This pet is still on a training cooldown.")]
    PetOnCooldown,
}