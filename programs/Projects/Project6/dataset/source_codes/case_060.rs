// (5) Energy Progress — 進行度・エネルギー管理
use anchor_lang::prelude::*;
declare_id!("11111111111111111111111111111111");

#[program]
pub mod energy_progress {
    use super::*;
    use CardRole::*;

    pub fn init_root(ctx: Context<InitRoot>, seed: u32) -> Result<()> {
        let r = &mut ctx.accounts.root;
        r.owner = ctx.accounts.owner.key();
        r.seed = seed;
        r.total_levels = 0;
        Ok(())
    }

    pub fn create_cards(ctx: Context<CreateCards>, role: CardRole, boost: u8) -> Result<()> {
        let r = &mut ctx.accounts.root;
        let p = &mut ctx.accounts.player;
        p.root = r.key();
        p.role = role;
        p.level = 1;
        p.energy = 100;
        let b = &mut ctx.accounts.booster;
        b.root = r.key();
        b.kind = boost.min(10);
        b.charge = 0;
        Ok(())
    }

    pub fn tick(ctx: Context<Tick>, deltas: Vec<i16>) -> Result<()> {
        let r = &mut ctx.accounts.root;
        let actor = &mut ctx.accounts.actor;
        let counter = &mut ctx.accounts.counter;
        let log = &mut ctx.accounts.progress_log;

        let mut energy_delta: i32 = 0;
        let mut mix: u32 = 0;
        for d in deltas {
            energy_delta += d as i32;
            mix = mix.rotate_left(2) ^ (d as u32 as u32);
        }
        let adj = if energy_delta >= 0 { energy_delta as u32 } else { 0 };

        if actor.role == Player {
            actor.energy = actor.energy.saturating_add((adj & 0xFF) as u16);
            counter.energy = counter.energy.saturating_add(((mix >> 3) & 0x7F) as u16);
            actor.level = actor.level.saturating_add(1);
            r.total_levels = r.total_levels.saturating_add(1);
            msg!("Player path: adj={}, mix={}, e_a={}, e_c={}", adj, mix, actor.energy, counter.energy);
        } else {
            actor.energy = actor.energy.saturating_add(((mix >> 2) & 0x3F) as u16);
            counter.energy = counter.energy.saturating_add((adj & 0x7F) as u16);
            counter.level = counter.level.saturating_add(1);
            r.total_levels = r.total_levels.saturating_add(1);
            msg!("Booster path: adj={}, mix={}, e_a={}, e_c={}", adj, mix, actor.energy, counter.energy);
        }

        // 近似平方根メトリクス
        let mut x = (r.total_levels as u128).max(1);
        let mut i = 0;
        while i < 3 {
            x = (x + (r.total_levels as u128 / x)).max(1) / 2;
            i += 1;
        }
        log.root = r.key();
        log.metric = (x as u32).min(1_000_000);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRoot<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 4)]
    pub root: Account<'info, Root>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateCards<'info> {
    #[account(mut)]
    pub root: Account<'info, Root>,
    #[account(init, payer = payer, space = 8 + 32 + 1 + 2 + 2)]
    pub player: Account<'info, PlayerCard>,
    #[account(init, payer = payer, space = 8 + 32 + 1 + 1)]
    pub booster: Account<'info, BoosterCard>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 同一親 + 役割不一致
#[derive(Accounts)]
pub struct Tick<'info> {
    #[account(mut)]
    pub root: Account<'info, Root>,
    #[account(mut, has_one = root)]
    pub progress_log: Account<'info, ProgressLog>,
    #[account(
        mut,
        has_one = root,
        constraint = actor.role != counter.role @ ErrCode::CosplayBlocked
    )]
    pub actor: Account<'info, PlayerCard>,
    #[account(mut, has_one = root)]
    pub counter: Account<'info, PlayerCard>,
}

#[account]
pub struct Root {
    pub owner: Pubkey,
    pub seed: u32,
    pub total_levels: u32,
}

#[account]
pub struct PlayerCard {
    pub root: Pubkey,
    pub role: CardRole,
    pub level: u16,
    pub energy: u16,
}

#[account]
pub struct BoosterCard {
    pub root: Pubkey,
    pub kind: u8,
    pub charge: u8,
}

#[account]
pub struct ProgressLog {
    pub root: Pubkey,
    pub metric: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum CardRole {
    Player,
    Booster,
}

#[error_code]
pub enum ErrCode {
    #[msg("Type cosplay prevented in energy tick.")]
    CosplayBlocked,
}
