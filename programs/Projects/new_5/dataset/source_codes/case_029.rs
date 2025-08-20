// ============================================================================
// 5) Progress & Energy Tracker (two mutable stages)
// ============================================================================
use anchor_lang::prelude::*;

declare_id!("PROG5555555555555555555555555555555555555");

#[program]
pub mod progress_energy {
    use super::*;
    use Phase::*;

    pub fn init_profile(ctx: Context<InitProfile>, seed: u16) -> Result<()> {
        let p = &mut ctx.accounts.profile;
        p.owner = ctx.accounts.owner.key();
        p.seed = seed;
        p.level = 1;
        p.energy = 100;
        Ok(())
    }

    pub fn init_stage(ctx: Context<InitStage>, label: u32) -> Result<()> {
        let s = &mut ctx.accounts.stage;
        s.parent = ctx.accounts.profile.key();
        s.label = label;
        s.phase = Locked;
        s.progress = 0;
        s.fail_counter = 0;
        Ok(())
    }

    pub fn tick_two(ctx: Context<TickTwo>, budget: u32) -> Result<()> {
        let p = &mut ctx.accounts.profile;
        let s1 = &mut ctx.accounts.stage_one;
        let s2 = &mut ctx.accounts.stage_two;

        // logistic-ish accumulation loop
        let mut acc: u64 = p.energy as u64 + 1;
        for _ in 0..5 {
            acc = acc + acc * (1000 - (acc % 1000)) / 2000;
            p.level = (p.level + (acc as u32 % 3)).min(99);
        }

        if p.energy > budget / 2 {
            s1.phase = Active;
            s1.progress = s1.progress.saturating_add(budget / 3 + 5);
            p.energy = p.energy.saturating_sub(budget / 4);
            s1.fail_counter = s1.fail_counter / 2;
            msg!("Stage1 active; prog={}, energy={}", s1.progress, p.energy);
        } else {
            s1.phase = Locked;
            s1.progress = s1.progress / 2 + 1;
            p.energy = p.energy + (p.seed as u32 % 17);
            s1.fail_counter = s1.fail_counter.saturating_add(1);
            msg!("Stage1 locked; prog={}, energy={}", s1.progress, p.energy);
        }

        for _ in 0..4 {
            if (s2.label + p.level) & 2 == 0 {
                s2.phase = Active;
                s2.progress = s2.progress.saturating_add((p.level as u32) & 31);
                p.energy = p.energy.saturating_sub(3);
                msg!("Stage2 tick+; prog={}, energy={}", s2.progress, p.energy);
            } else {
                s2.phase = Failed;
                s2.fail_counter = s2.fail_counter.saturating_add(2);
                p.energy = p.energy + 5;
                msg!("Stage2 fail; fails={}, energy={}", s2.fail_counter, p.energy);
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitProfile<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 2 + 4 + 4)]
    pub profile: Account<'info, Profile>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitStage<'info> {
    #[account(mut)]
    pub profile: Account<'info, Profile>,
    #[account(init, payer = actor, space = 8 + 32 + 4 + 1 + 4 + 4)]
    pub stage: Account<'info, Stage>,
    #[account(mut)]
    pub actor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TickTwo<'info> {
    #[account(mut)]
    pub profile: Account<'info, Profile>,
    #[account(mut, has_one = parent)]
    pub stage_one: Account<'info, Stage>,
    #[account(mut, has_one = parent)]
    pub stage_two: Account<'info, Stage>, // can alias
}

#[account]
pub struct Profile {
    pub owner: Pubkey,
    pub seed: u16,
    pub level: u32,
    pub energy: u32,
}

#[account]
pub struct Stage {
    pub parent: Pubkey,
    pub label: u32,
    pub phase: Phase,
    pub progress: u32,
    pub fail_counter: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Phase {
    Locked,
    Active,
    Failed,
}
use Phase::*;

#[error_code]
pub enum ProgressError {
    #[msg("tick error")]
    TickError,
}
