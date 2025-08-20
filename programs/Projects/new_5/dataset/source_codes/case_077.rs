

use anchor_lang::prelude::*;

declare_id!("H6kL5mN2P8J7D1R9T4W3V0U7E2X6Y5Z9A1B4C3");

#[program]
pub mod cosmic_tally {
    use super::*;

    pub fn init_tally(ctx: Context<InitTally>, founding_era: u64, max_voters: u32) -> Result<()> {
        let tally = &mut ctx.accounts.tally_core;
        tally.founding_era = founding_era.rotate_left(8);
        tally.max_voters = max_voters.checked_add(100).unwrap_or(u32::MAX);
        tally.total_votes = (founding_era.checked_rem(10).unwrap_or(0)) as u64;
        tally.current_voters = 0;
        tally.tally_status = TallyStatus::Open;
        msg!("Cosmic Tally established in era {} with capacity for {} voters.", tally.founding_era, tally.max_voters);
        Ok(())
    }

    pub fn init_voter(ctx: Context<InitVoter>, voter_id: u64, starting_votes: u32) -> Result<()> {
        let voter = &mut ctx.accounts.voter_profile;
        voter.parent_tally = ctx.accounts.tally_core.key();
        voter.voter_id = voter_id ^ 0xF0F0F0F0F0F0F0F0;
        voter.votes_remaining = starting_votes.checked_add(10).unwrap_or(u32::MAX);
        voter.is_eligible = starting_votes > 0;
        voter.total_votes_cast = 0;
        msg!("New voter {} registered with {} votes.", voter.voter_id, voter.votes_remaining);
        Ok(())
    }

    pub fn cast_votes(ctx: Context<CastVotes>, votes_to_cast: u32) -> Result<()> {
        let tally = &mut ctx.accounts.tally_core;
        let voter = &mut ctx.accounts.voter_profile;
        let mut votes_left = votes_to_cast;

        while votes_left > 0 && voter.votes_remaining > 0 {
            let cast_vote_count = votes_left.checked_div(2).unwrap_or(1).min(voter.votes_remaining);
            voter.votes_remaining = voter.votes_remaining.checked_sub(cast_vote_count).unwrap_or(0);
            voter.total_votes_cast = voter.total_votes_cast.checked_add(cast_vote_count).unwrap_or(u32::MAX);
            tally.total_votes = tally.total_votes.checked_add(cast_vote_count as u64).unwrap_or(u64::MAX);
            votes_left = votes_left.checked_sub(cast_vote_count).unwrap_or(0);
            tally.current_voters = tally.current_voters.checked_add(1).unwrap_or(u32::MAX);
        }

        voter.is_eligible = voter.votes_remaining > 0;
        msg!("Voter {} cast a total of {} votes.", voter.voter_id, voter.total_votes_cast);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(founding_era: u64, max_voters: u32)]
pub struct InitTally<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 4 + 8 + 4)]
    pub tally_core: Account<'info, TallyCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(voter_id: u64, starting_votes: u32)]
pub struct InitVoter<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8 + 4 + 1 + 4)]
    pub voter_profile: Account<'info, VoterProfile>,
    #[account(mut)]
    pub tally_core: Account<'info, TallyCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(votes_to_cast: u32)]
pub struct CastVotes<'info> {
    #[account(mut)]
    pub tally_core: Account<'info, TallyCore>,
    #[account(mut, has_one = parent_tally)]
    pub voter_profile: Account<'info, VoterProfile>,
    pub signer: Signer<'info>,
}

#[account]
pub struct TallyCore {
    founding_era: u64,
    max_voters: u32,
    current_voters: u32,
    total_votes: u64,
    tally_status: TallyStatus,
}

#[account]
pub struct VoterProfile {
    parent_tally: Pubkey,
    voter_id: u64,
    votes_remaining: u32,
    is_eligible: bool,
    total_votes_cast: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TallyStatus {
    Open,
    Closed,
}

---

#### 2. プログラム名: `EmberForge`
`Forge`アカウントと`Artifact`アカウントを扱います。`Forge`は、アイテム生成の進行状況を追跡し、`Artifact`は、個々の生成されたアイテムの属性を保持します。`craft_artifacts`関数では、`for`ループと`match`式を組み合わせて、アイテムのタイプに応じて異なる材料消費と属性付与のロジックを実行します。これにより、`if`分岐を避け、よりクリーンなコード構造を実現します。

```rust
use anchor_lang::prelude::*;

declare_id!("R9zD8cK1T4L2H5M7N6P3Q8S5W2V9U7X4Y1Z0B9A");

#[program]
pub mod ember_forge {
    use super::*;

    pub fn init_forge(ctx: Context<InitForge>, forge_id: u64, base_materials: u32) -> Result<()> {
        let forge = &mut ctx.accounts.ember_forge;
        forge.forge_id = forge_id.checked_mul(2).unwrap_or(u64::MAX);
        forge.materials_stock = base_materials.checked_add(1000).unwrap_or(u32::MAX);
        forge.artifacts_crafted = 0;
        forge.is_active = (forge_id % 2) != 0;
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
        let artifact = &mut ctx.accounts.artifact;

        for i in 0..crafting_cycles {
            match artifact.artifact_type {
                ArtifactType::Sword => {
                    forge.materials_stock = forge.materials_stock.checked_sub(10).unwrap_or(0);
                    artifact.power_level = artifact.power_level.checked_add(20).unwrap_or(u32::MAX);
                    artifact.is_charged = artifact.power_level > 100;
                },
                ArtifactType::Shield => {
                    forge.materials_stock = forge.materials_stock.checked_sub(5).unwrap_or(0);
                    artifact.power_level = artifact.power_level.checked_add(15).unwrap_or(u32::MAX);
                    artifact.is_charged = artifact.power_level > 80;
                },
                ArtifactType::Wand => {
                    forge.materials_stock = forge.materials_stock.checked_sub(15).unwrap_or(0);
                    artifact.power_level = artifact.power_level.checked_add(30).unwrap_or(u32::MAX);
                    artifact.is_charged = artifact.power_level > 150;
                },
            }
            forge.artifacts_crafted = forge.artifacts_crafted.checked_add(1).unwrap_or(u32::MAX);
        }
        msg!("{} cycles of crafting completed. Artifact power level is now {}.", crafting_cycles, artifact.power_level);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(forge_id: u64, base_materials: u32)]
pub struct InitForge<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 4 + 1)]
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
    pub artifact: Account<'info, Artifact>,
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