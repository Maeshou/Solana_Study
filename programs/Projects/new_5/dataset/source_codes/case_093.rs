use anchor_lang::prelude::*;
use rand::Rng;

declare_id!("A1zX9yW8V7U6T5S4R3Q2P1O0N9M8L7K6J5I4H3");

const CIRCLE_ID_BONUS: u64 = 9876;
const INITIAL_AETHER_MULTIPLIER: u64 = 2;
const INITIAL_AETHER_THRESHOLD: u64 = 1000;
const AETHER_COST_ELEMENTALIST: u64 = 20;
const AETHER_COST_ILLUSIONIST: u64 = 15;
const AETHER_COST_NECROMANCER: u64 = 30;
const MASTERY_GAIN_ELEMENTALIST: u32 = 1;
const MASTERY_GAIN_ILLUSIONIST: u32 = 2;
const MASTERY_GAIN_NECROMANCER: u32 = 3;
const MASTERY_THRESHOLD_ELEMENTALIST: u32 = 50;
const MASTERY_THRESHOLD_ILLUSIONIST: u32 = 80;
const MASTERY_THRESHOLD_NECROMANCER: u32 = 100;

#[program]
pub mod aether_mages {
    use super::*;

    pub fn init_circle(ctx: Context<InitCircle>, circle_id: u64, initial_aether: u64) -> Result<()> {
        let circle = &mut ctx.accounts.magic_circle;
        circle.circle_id = circle_id + CIRCLE_ID_BONUS;
        circle.aether_pool = initial_aether * INITIAL_AETHER_MULTIPLIER;
        circle.active_casters = 0;
        circle.is_active = circle.aether_pool > INITIAL_AETHER_THRESHOLD;
        msg!("Magic Circle {} created with {} aether.", circle.circle_id, circle.aether_pool);
        Ok(())
    }

    pub fn init_caster(ctx: Context<InitCaster>, caster_id: u64, caster_type: CasterType) -> Result<()> {
        let caster = &mut ctx.accounts.spellcaster;
        caster.parent_circle = ctx.accounts.magic_circle.key();
        caster.caster_id = caster_id ^ 0xFF00FF00FF00FF00;
        caster.caster_type = caster_type;
        caster.mastery_level = 0;
        caster.is_casting = false;
        msg!("Spellcaster {} of type {:?} joins the circle.", caster.caster_id, caster.caster_type);
        Ok(())
    }

    pub fn cast_spells(ctx: Context<CastSpells>, cycles: u32) -> Result<()> {
        let circle = &mut ctx.accounts.magic_circle;
        let master_caster = &mut ctx.accounts.master_caster;
        let apprentice_caster = &mut ctx.accounts.apprentice_caster;

        for _i in 0..cycles {
            // master_casterの呪文詠唱
            match master_caster.caster_type {
                CasterType::Elementalist => {
                    circle.aether_pool = circle.aether_pool.saturating_sub(AETHER_COST_ELEMENTALIST);
                    master_caster.mastery_level = master_caster.mastery_level.saturating_add(MASTERY_GAIN_ELEMENTALIST);
                    master_caster.is_casting = master_caster.mastery_level < MASTERY_THRESHOLD_ELEMENTALIST;
                },
                CasterType::Illusionist => {
                    circle.aether_pool = circle.aether_pool.saturating_sub(AETHER_COST_ILLUSIONIST);
                    master_caster.mastery_level = master_caster.mastery_level.saturating_add(MASTERY_GAIN_ILLUSIONIST);
                    master_caster.is_casting = master_caster.mastery_level < MASTERY_THRESHOLD_ILLUSIONIST;
                },
                CasterType::Necromancer => {
                    circle.aether_pool = circle.aether_pool.saturating_sub(AETHER_COST_NECROMANCER);
                    master_caster.mastery_level = master_caster.mastery_level.saturating_add(MASTERY_GAIN_NECROMANCER);
                    master_caster.is_casting = master_caster.mastery_level < MASTERY_THRESHOLD_NECROMANCER;
                },
            }

            // apprentice_casterの呪文詠唱
            match apprentice_caster.caster_type {
                CasterType::Elementalist => {
                    circle.aether_pool = circle.aether_pool.saturating_sub(AETHER_COST_ELEMENTALIST);
                    apprentice_caster.mastery_level = apprentice_caster.mastery_level.saturating_add(MASTERY_GAIN_ELEMENTALIST);
                    apprentice_caster.is_casting = apprentice_caster.mastery_level < MASTERY_THRESHOLD_ELEMENTALIST;
                },
                CasterType::Illusionist => {
                    circle.aether_pool = circle.aether_pool.saturating_sub(AETHER_COST_ILLUSIONIST);
                    apprentice_caster.mastery_level = apprentice_caster.mastery_level.saturating_add(MASTERY_GAIN_ILLUSIONIST);
                    apprentice_caster.is_casting = apprentice_caster.mastery_level < MASTERY_THRESHOLD_ILLUSIONIST;
                },
                CasterType::Necromancer => {
                    circle.aether_pool = circle.aether_pool.saturating_sub(AETHER_COST_NECROMANCER);
                    apprentice_caster.mastery_level = apprentice_caster.mastery_level.saturating_add(MASTERY_GAIN_NECROMANCER);
                    apprentice_caster.is_casting = apprentice_caster.mastery_level < MASTERY_THRESHOLD_NECROMANCER;
                },
            }

            circle.active_casters = (master_caster.is_casting as u32) + (apprentice_caster.is_casting as u32);
        }

        circle.is_active = circle.aether_pool > INITIAL_AETHER_THRESHOLD;
        msg!("Spellcasters performed {} cycles of casting. Master mastery: {}, Apprentice mastery: {}.", 
            cycles, master_caster.mastery_level, apprentice_caster.mastery_level);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(circle_id: u64, initial_aether: u64)]
pub struct InitCircle<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 8 + 4 + 1)]
    pub magic_circle: Account<'info, MagicCircle>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(caster_id: u64, caster_type: CasterType)]
pub struct InitCaster<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8 + 1 + 4 + 1)]
    pub spellcaster: Account<'info, Spellcaster>,
    #[account(mut)]
    pub magic_circle: Account<'info, MagicCircle>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(cycles: u32)]
pub struct CastSpells<'info> {
    #[account(mut)]
    pub magic_circle: Account<'info, MagicCircle>,
    #[account(mut, has_one = parent_circle)]
    pub master_caster: Account<'info, Spellcaster>,
    #[account(mut, has_one = parent_circle)]
    pub apprentice_caster: Account<'info, Spellcaster>,
    pub signer: Signer<'info>,
}

#[account]
pub struct MagicCircle {
    circle_id: u64,
    aether_pool: u64,
    active_casters: u32,
    is_active: bool,
}

#[account]
pub struct Spellcaster {
    parent_circle: Pubkey,
    caster_id: u64,
    caster_type: CasterType,
    mastery_level: u32,
    is_casting: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum CasterType {
    Elementalist,
    Illusionist,
    Necromancer,
}