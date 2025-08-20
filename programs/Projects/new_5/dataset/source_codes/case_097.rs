use anchor_lang::prelude::*;

declare_id!("A2F4R6H8K0J9N1M3P5Q7S9V1W3Y5Z7A0B2C4D6");

const BASE_POWER_BONUS: u32 = 100;
const MERGE_SUCCESS_THRESHOLD: u32 = 100;
const MIN_POWER_LEVEL: u32 = 50;

#[program]
pub mod arcane_foundry {
    use super::*;

    pub fn init_foundry(ctx: Context<InitFoundry>, foundry_id: u64, power_level: u32) -> Result<()> {
        let foundry = &mut ctx.accounts.foundry_core;
        foundry.foundry_id = foundry_id + 500;
        foundry.total_power = power_level.saturating_add(BASE_POWER_BONUS);
        foundry.runes_merged = 0;
        foundry.is_operational = foundry.total_power > MIN_POWER_LEVEL;
        msg!("Arcane Foundry {} initialized with {} power.", foundry.foundry_id, foundry.total_power);
        Ok(())
    }

    pub fn init_rune(ctx: Context<InitRune>, rune_id: u64, base_power: u32) -> Result<()> {
        let rune = &mut ctx.accounts.magic_rune;
        rune.parent_foundry = ctx.accounts.foundry_core.key();
        rune.rune_id = rune_id * 7;
        rune.power_level = base_power;
        rune.is_merged = false;
        msg!("New rune {} created with {} base power.", rune.rune_id, rune.power_level);
        Ok(())
    }

    pub fn merge_runes(ctx: Context<MergeRunes>) -> Result<()> {
        let foundry = &mut ctx.accounts.foundry_core;
        let first_rune = &mut ctx.accounts.first_rune;
        let second_rune = &mut ctx.accounts.second_rune;

        if first_rune.is_merged || second_rune.is_merged {
            return Err(ProgramError::Custom(1).into()); // Error: One or both runes already merged
        }
        
        let combined_power = first_rune.power_level.saturating_add(second_rune.power_level);
        
        if combined_power > MERGE_SUCCESS_THRESHOLD {
            // 合成成功
            foundry.total_power = foundry.total_power.saturating_add(combined_power);
            first_rune.power_level = combined_power;
            second_rune.power_level = 0; // マージされたルーンは消滅
            first_rune.is_merged = true;
            second_rune.is_merged = true;
            foundry.runes_merged = foundry.runes_merged.saturating_add(2);
            msg!("Rune merge successful! New combined power: {}.", combined_power);
        } else {
            // 合成失敗
            foundry.total_power = foundry.total_power.saturating_sub(combined_power / 2);
            msg!("Rune merge failed. Foundry power reduced.");
        }
        foundry.is_operational = foundry.total_power > MIN_POWER_LEVEL;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(foundry_id: u64, power_level: u32)]
pub struct InitFoundry<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 4 + 4 + 1)]
    pub foundry_core: Account<'info, FoundryCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(rune_id: u64, base_power: u32)]
pub struct InitRune<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8 + 4 + 1)]
    pub magic_rune: Account<'info, MagicRune>,
    #[account(mut)]
    pub foundry_core: Account<'info, FoundryCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MergeRunes<'info> {
    #[account(mut)]
    pub foundry_core: Account<'info, FoundryCore>,
    #[account(mut, has_one = parent_foundry)]
    pub first_rune: Account<'info, MagicRune>,
    #[account(mut, has_one = parent_foundry)]
    pub second_rune: Account<'info, MagicRune>,
    pub signer: Signer<'info>,
}

#[account]
pub struct FoundryCore {
    foundry_id: u64,
    total_power: u32,
    runes_merged: u32,
    is_operational: bool,
}

#[account]
pub struct MagicRune {
    parent_foundry: Pubkey,
    rune_id: u64,
    power_level: u32,
    is_merged: bool,
}