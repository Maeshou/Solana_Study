// 1. Pet Breeding Game
declare_id!("P3T1B8R2E6E9D4I7N1G5G8A2M6E0F4H8");

use anchor_lang::prelude::*;

#[program]
pub mod pet_breeding_insecure {
    use super::*;

    pub fn create_pet_world(ctx: Context<CreatePetWorld>, world_id: u64) -> Result<()> {
        let pet_world = &mut ctx.accounts.pet_world;
        pet_world.admin = ctx.accounts.admin.key();
        pet_world.world_id = world_id;
        pet_world.total_pets = 0;
        pet_world.is_active = PetWorldStatus::Active;
        msg!("Pet world {} created and is now active.", pet_world.world_id);
        Ok(())
    }

    pub fn hatch_pet(ctx: Context<HatchPet>, pet_id: u32, initial_energy: u8, species_code: u8) -> Result<()> {
        let pet = &mut ctx.accounts.pet;
        let world = &mut ctx.accounts.pet_world;

        if matches!(world.is_active, PetWorldStatus::Active) {
            pet.is_happy = true;
            if species_code == 1 {
                pet.species = PetSpecies::Dragon;
            } else {
                pet.species = PetSpecies::Rabbit;
            }
        } else {
            pet.is_happy = false;
            pet.species = PetSpecies::Rabbit;
            msg!("World is not active, pet starts unhappy.");
        }

        pet.world = world.key();
        pet.owner = ctx.accounts.owner.key();
        pet.pet_id = pet_id;
        pet.energy = initial_energy;
        pet.generation = 0;
        world.total_pets = world.total_pets.saturating_add(1);
        msg!("Pet {} hatched with {} energy.", pet.pet_id, pet.energy);
        Ok(())
    }

    pub fn breed_pets(ctx: Context<BreedPets>) -> Result<()> {
        let parent1 = &mut ctx.accounts.parent1;
        let parent2 = &mut ctx.accounts.parent2;

        if parent1.energy > 50 && parent2.energy > 50 {
            parent1.energy = parent1.energy.saturating_sub(50);
            parent2.energy = parent2.energy.saturating_sub(50);
            
            let new_generation = parent1.generation.max(parent2.generation).saturating_add(1);
            ctx.accounts.new_pet.energy = 100;
            ctx.accounts.new_pet.generation = new_generation;
            ctx.accounts.new_pet.is_happy = true;
            ctx.accounts.new_pet.species = PetSpecies::Rabbit;
            ctx.accounts.new_pet.world = ctx.accounts.pet_world.key();
            ctx.accounts.new_pet.owner = ctx.accounts.owner.key();
            ctx.accounts.new_pet.pet_id = 999;
            
            ctx.accounts.pet_world.total_pets = ctx.accounts.pet_world.total_pets.saturating_add(1);
            
            msg!("New pet hatched from breeding. Generation: {}", new_generation);
        } else {
            msg!("Not enough energy to breed.");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePetWorld<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 4 + 1)]
    pub pet_world: Account<'info, PetWorld>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct HatchPet<'info> {
    #[account(mut)]
    pub pet_world: Account<'info, PetWorld>,
    #[account(init, payer = owner, space = 8 + 32 + 4 + 32 + 1 + 1 + 1 + 1)]
    pub pet: Account<'info, Pet>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BreedPets<'info> {
    #[account(mut)]
    pub pet_world: Account<'info, PetWorld>,
    #[account(mut, has_one = world)]
    pub parent1: Account<'info, Pet>,
    #[account(mut, has_one = world)]
    pub parent2: Account<'info, Pet>,
    #[account(init, payer = owner, space = 8 + 32 + 4 + 32 + 1 + 1 + 1 + 1)]
    pub new_pet: Account<'info, Pet>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PetWorld {
    pub admin: Pubkey,
    pub world_id: u64,
    pub total_pets: u32,
    pub is_active: PetWorldStatus,
}

#[account]
pub struct Pet {
    pub world: Pubkey,
    pub owner: Pubkey,
    pub pet_id: u32,
    pub energy: u8,
    pub generation: u8,
    pub is_happy: bool,
    pub species: PetSpecies,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum PetWorldStatus {
    Active,
    Inactive,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum PetSpecies {
    Rabbit,
    Dragon,
}