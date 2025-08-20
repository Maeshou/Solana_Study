use anchor_lang::prelude::*;

declare_id!("R9zD8cK1T4L2H5M7N6P3Q8S5W2V9U7X4Y1Z0B9A");

const INITIAL_MATERIAL_BONUS: u32 = 1000;
const MIN_ACTIVE_MATERIALS: u64 = 1000;
const SWORD_MATERIAL_COST: u32 = 10;
const SHIELD_MATERIAL_COST: u32 = 5;
const WAND_MATERIAL_COST: u32 = 15;
const SWORD_POWER_GAIN: u32 = 20;
const SHIELD_POWER_GAIN: u32 = 15;
const WAND_POWER_GAIN: u32 = 30;
const SWORD_CHARGE_THRESHOLD: u32 = 100;
const SHIELD_CHARGE_THRESHOLD: u32 = 80;
const WAND_CHARGE_THRESHOLD: u32 = 150;

#[program]
pub mod ember_forge {
    use super::*;

    pub fn init_forge(ctx: Context<InitForge>, forge_id: u64, base_materials: u32) -> Result<()> {
        let forge = &mut ctx.accounts.ember_forge;
        forge.forge_id = forge_id * 2;
        forge.materials_stock = base_materials + INITIAL_MATERIAL_BONUS;
        forge.artifacts_crafted = 0;
        forge.is_active = forge.materials_stock as u64 > MIN_ACTIVE_MATERIALS;
        msg!("Ember Forge {} established with {} materials.", forge.forge_id, forge.materials_stock);
        Ok(())
    }

    pub fn init_artifact(ctx: Context<InitArtifact>, artifact_id: u64, artifact_type: ArtifactType) -> Result<()> {
        let artifact = &mut ctx.accounts.artifact;
        artifact.parent_forge = ctx.accounts.ember_forge.key();
        artifact.artifact_id = artifact_id ^ 0xA5A5A5A5A5A5A5A5;
        artifact.artifact_type = artifact_type;
        artifact.power_level = 0;
        artifact.is_charged = false;
        msg!("New artifact {} of type {:?} created.", artifact.artifact_id, artifact.artifact_type);
        Ok(())
    }

    pub fn craft_artifacts(ctx: Context<CraftArtifacts>, crafting_cycles: u32) -> Result<()> {
        let forge = &mut ctx.accounts.ember_forge;
        let weapon_artifact = &mut ctx.accounts.weapon_artifact;
        let shield_artifact = &mut ctx.accounts.shield_artifact;

        for _i in 0..crafting_cycles {
            // weapon_artifactのクラフト処理
            match weapon_artifact.artifact_type {
                ArtifactType::Sword => {
                    forge.materials_stock = forge.materials_stock.saturating_sub(SWORD_MATERIAL_COST);
                    weapon_artifact.power_level = weapon_artifact.power_level.saturating_add(SWORD_POWER_GAIN);
                    weapon_artifact.is_charged = weapon_artifact.power_level > SWORD_CHARGE_THRESHOLD;
                },
                ArtifactType::Shield => {
                    forge.materials_stock = forge.materials_stock.saturating_sub(SHIELD_MATERIAL_COST);
                    weapon_artifact.power_level = weapon_artifact.power_level.saturating_add(SHIELD_POWER_GAIN);
                    weapon_artifact.is_charged = weapon_artifact.power_level > SHIELD_CHARGE_THRESHOLD;
                },
                ArtifactType::Wand => {
                    forge.materials_stock = forge.materials_stock.saturating_sub(WAND_MATERIAL_COST);
                    weapon_artifact.power_level = weapon_artifact.power_level.saturating_add(WAND_POWER_GAIN);
                    weapon_artifact.is_charged = weapon_artifact.power_level > WAND_CHARGE_THRESHOLD;
                },
            }

            // shield_artifactのクラフト処理
            match shield_artifact.artifact_type {
                ArtifactType::Sword => {
                    forge.materials_stock = forge.materials_stock.saturating_sub(SWORD_MATERIAL_COST);
                    shield_artifact.power_level = shield_artifact.power_level.saturating_add(SWORD_POWER_GAIN);
                    shield_artifact.is_charged = shield_artifact.power_level > SWORD_CHARGE_THRESHOLD;
                },
                ArtifactType::Shield => {
                    forge.materials_stock = forge.materials_stock.saturating_sub(SHIELD_MATERIAL_COST);
                    shield_artifact.power_level = shield_artifact.power_level.saturating_add(SHIELD_POWER_GAIN);
                    shield_artifact.is_charged = shield_artifact.power_level > SHIELD_CHARGE_THRESHOLD;
                },
                ArtifactType::Wand => {
                    forge.materials_stock = forge.materials_stock.saturating_sub(WAND_MATERIAL_COST);
                    shield_artifact.power_level = shield_artifact.power_level.saturating_add(WAND_POWER_GAIN);
                    shield_artifact.is_charged = shield_artifact.power_level > WAND_CHARGE_THRESHOLD;
                },
            }
            forge.artifacts_crafted = forge.artifacts_crafted.saturating_add(2);
        }
        msg!("{} cycles of crafting completed. Weapon power level: {}, Shield power level: {}.", 
            crafting_cycles, weapon_artifact.power_level, shield_artifact.power_level);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(forge_id: u64, base_materials: u32)]
pub struct InitForge<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 4 + 4 + 1)]
    pub ember_forge: Account<'info, EmberForge>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(artifact_id: u64, artifact_type: ArtifactType)]
pub struct InitArtifact<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8 + 1 + 4 + 1)]
    pub artifact: Account<'info, Artifact>,
    #[account(mut)]
    pub ember_forge: Account<'info, EmberForge>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(crafting_cycles: u32)]
pub struct CraftArtifacts<'info> {
    #[account(mut)]
    pub ember_forge: Account<'info, EmberForge>,
    #[account(mut, has_one = parent_forge)]
    pub weapon_artifact: Account<'info, Artifact>,
    #[account(mut, has_one = parent_forge)]
    pub shield_artifact: Account<'info, Artifact>,
    pub signer: Signer<'info>,
}

#[account]
pub struct EmberForge {
    forge_id: u64,
    materials_stock: u32,
    artifacts_crafted: u32,
    is_active: bool,
}

#[account]
pub struct Artifact {
    parent_forge: Pubkey,
    artifact_id: u64,
    artifact_type: ArtifactType,
    power_level: u32,
    is_charged: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ArtifactType {
    Sword,
    Shield,
    Wand,
}