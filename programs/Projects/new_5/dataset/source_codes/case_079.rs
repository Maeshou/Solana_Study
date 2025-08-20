use anchor_lang::prelude::*;
use rand::Rng;

declare_id!("A1zX9yW8V7U6T5S4R3Q2P1O0N9M8L7K6J5I4H3");

#[program]
pub mod aether_mages {
    use super::*;

    pub fn init_circle(ctx: Context<InitCircle>, circle_id: u64, initial_aether: u64) -> Result<()> {
        let circle = &mut ctx.accounts.magic_circle;
        circle.circle_id = circle_id.checked_add(9876).unwrap_or(u64::MAX);
        circle.aether_pool = initial_aether.checked_mul(2).unwrap_or(u64::MAX);
        circle.active_casters = 0;
        circle.is_active = circle.aether_pool > 1000;
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
        let caster = &mut ctx.accounts.spellcaster;

        for _i in 0..cycles {
            match caster.caster_type {
                CasterType::Elementalist => {
                    circle.aether_pool = circle.aether_pool.checked_sub(20).unwrap_or(0);
                    caster.mastery_level = caster.mastery_level.checked_add(1).unwrap_or(u32::MAX);
                    caster.is_casting = caster.mastery_level < 50;
                },
                CasterType::Illusionist => {
                    circle.aether_pool = circle.aether_pool.checked_sub(15).unwrap_or(0);
                    caster.mastery_level = caster.mastery_level.checked_add(2).unwrap_or(u32::MAX);
                    caster.is_casting = caster.mastery_level < 80;
                },
                CasterType::Necromancer => {
                    circle.aether_pool = circle.aether_pool.checked_sub(30).unwrap_or(0);
                    caster.mastery_level = caster.mastery_level.checked_add(3).unwrap_or(u32::MAX);
                    caster.is_casting = caster.mastery_level < 100;
                },
            }
            circle.active_casters = circle.active_casters.checked_add(1).unwrap_or(u32::MAX);
        }

        circle.is_active = circle.aether_pool > 1000;
        msg!("Spellcaster {} performed {} cycles of casting.", caster.caster_id, cycles);
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
    pub spellcaster: Account<'info, Spellcaster>,
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