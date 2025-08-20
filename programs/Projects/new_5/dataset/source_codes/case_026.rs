// ============================================================================
// 2) Blacksmith Forge (two mutable items in same instruction)
// ============================================================================
use anchor_lang::prelude::*;

declare_id!("FORGE2222222222222222222222222222222222222");

#[program]
pub mod blacksmith_forge {
    use super::*;
    use ForgeStep::*;

    pub fn init_smith(ctx: Context<InitSmith>, tier: u8) -> Result<()> {
        let s = &mut ctx.accounts.smithy;
        s.owner = ctx.accounts.owner.key();
        s.tier = tier;
        s.heat = 300;
        s.total_work = 0;
        Ok(())
    }

    pub fn init_item(ctx: Context<InitItem>, code: u32) -> Result<()> {
        let it = &mut ctx.accounts.item;
        it.parent = ctx.accounts.smithy.key();
        it.code = code;
        it.step = Raw;
        it.durability = 500;
        it.quality = 0;
        Ok(())
    }

    pub fn process_dual(ctx: Context<ProcessDual>, fuel: u32) -> Result<()> {
        let s = &mut ctx.accounts.smithy;
        let left = &mut ctx.accounts.first_item;
        let right = &mut ctx.accounts.second_item;

        // triangular-wave like accumulation
        let mut phase: i32 = (s.heat as i32) % 40 - 20;
        for _ in 0..5 {
            phase = if phase < 0 { -phase } else { 20 - phase };
            let delta = (phase.abs() as u32).min(25);
            s.total_work = s.total_work.saturating_add(delta as u64);
            s.heat = s.heat.saturating_add(fuel / 10 + delta);
            if s.heat > 2000 {
                s.heat = 800 + (s.heat % 300);
            }
        }

        if left.durability > 0 {
            left.durability = left.durability.saturating_sub((fuel / 8) + 3);
            left.quality = left.quality.checked_add(s.tier as u32 + 1).unwrap_or(u32::MAX);
            left.step = Tempered;
            msg!("Left tempered; qual={}, dura={}", left.quality, left.durability);
        } else {
            left.quality = (left.quality / 2) + (s.tier as u32);
            left.step = Scrapped;
            s.heat = s.heat / 2 + 111;
            msg!("Left scrapped; heat={}, qual={}", s.heat, left.quality);
        }

        let mix = (right.code ^ (s.heat as u32)).rotate_left((s.tier % 31) as u32);
        if (mix & 0xFF) > 127 {
            right.step = Polished;
            right.quality = right.quality.saturating_add((mix & 255) + 5);
            s.total_work = s.total_work.saturating_add(7);
            msg!("Right polished; q={}, work={}", right.quality, s.total_work);
        } else {
            right.step = Tempered;
            right.durability = right.durability.saturating_add((mix % 41) + 1);
            s.heat = s.heat.saturating_sub((mix % 13) as u32);
            msg!("Right tempered alt; dura={}, heat={}", right.durability, s.heat);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSmith<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 1 + 4 + 8)]
    pub smithy: Account<'info, Smithy>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitItem<'info> {
    #[account(mut)]
    pub smithy: Account<'info, Smithy>,
    #[account(init, payer = user, space = 8 + 32 + 4 + 1 + 4 + 4)]
    pub item: Account<'info, Item>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessDual<'info> {
    #[account(mut)]
    pub smithy: Account<'info, Smithy>,
    #[account(mut, has_one = parent)]
    pub first_item: Account<'info, Item>,
    #[account(mut, has_one = parent)]
    pub second_item: Account<'info, Item>, // can alias first_item -> DMA risk
}

#[account]
pub struct Smithy {
    pub owner: Pubkey,
    pub tier: u8,
    pub heat: u32,
    pub total_work: u64,
}

#[account]
pub struct Item {
    pub parent: Pubkey,
    pub code: u32,
    pub step: ForgeStep,
    pub durability: u32,
    pub quality: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ForgeStep {
    Raw,
    Tempered,
    Polished,
    Scrapped,
}
use ForgeStep::*;

#[error_code]
pub enum ForgeError {
    #[msg("forge failure")]
    ForgeFailure,
}
