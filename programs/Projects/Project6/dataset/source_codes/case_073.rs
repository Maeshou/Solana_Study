// (8) FarmTycoon — 農園（作業者/作業者の不一致 + サイロ）
use anchor_lang::prelude::*;
declare_id!("11111111111111111111111111111111");

#[program]
pub mod farm_tycoon {
    use super::*;
    use Worker::*;

    pub fn init_farm(ctx: Context<InitFarm>, code: u32) -> Result<()> {
        let f = &mut ctx.accounts.farm;
        f.owner = ctx.accounts.owner.key();
        f.code = code;
        f.harvests = 0;
        Ok(())
    }

    pub fn recruit(ctx: Context<Recruit>, w: Worker, tag: u8) -> Result<()> {
        let f = &mut ctx.accounts.farm;
        let a = &mut ctx.accounts.worker_a;
        a.farm = f.key();
        a.kind = w;
        a.tag = tag;
        a.skill = 0;
        let s = &mut ctx.accounts.silo;
        s.farm = f.key();
        s.grain = 0;
        Ok(())
    }

    pub fn harvest(ctx: Context<Harvest>, plots: Vec<u16>) -> Result<()> {
        let f = &mut ctx.accounts.farm;
        let x = &mut ctx.accounts.actor;
        let y = &mut ctx.accounts.partner;
        let s = &mut ctx.accounts.silo;
        let l = &mut ctx.accounts.farm_log;

        let mut sum: u32 = 0;
        let mut mix: u16 = 0;
        for p in plots {
            sum = sum.saturating_add((p & 0x3FF) as u32);
            mix ^= p.rotate_left(1) ^ 0x3333;
        }
        let base = sum + (mix as u32 & 0xFF);

        if x.kind == Harvester {
            x.skill = x.skill.saturating_add((base / 2) as u16);
            y.skill = y.skill.saturating_add(((mix >> 2) & 0x3F) as u16);
            s.grain = s.grain.saturating_add((base / 4) as u32);
            f.harvests = f.harvests.saturating_add(1);
            msg!("Harvester: base={}, xS={}, yS={}, grain={}", base, x.skill, y.skill, s.grain);
        } else {
            x.skill = x.skill.saturating_add((base / 3) as u16);
            y.skill = y.skill.saturating_add(((mix >> 1) & 0x7F) as u16);
            s.grain = s.grain.saturating_add((base / 5) as u32);
            f.harvests = f.harvests.saturating_add(1);
            msg!("Planter/Waterer: base={}, xS={}, yS={}, grain={}", base, x.skill, y.skill, s.grain);
        }

        let mut t = (f.harvests as u128).max(1);
        let mut i = 0;
        while i < 3 { t = (t + (f.harvests as u128 / t)).max(1) / 2; i += 1; }
        l.farm = f.key();
        l.index = (t as u32).min(1_000_000);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitFarm<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 4)]
    pub farm: Account<'info, Farm>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Recruit<'info> {
    #[account(mut)]
    pub farm: Account<'info, Farm>,
    #[account(init, payer = payer, space = 8 + 32 + 1 + 1 + 2)]
    pub worker_a: Account<'info, WorkerCard>,
    #[account(init, payer = payer, space = 8 + 32 + 4)]
    pub silo: Account<'info, Silo>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
// 同一親 + 作業者タグ不一致
#[derive(Accounts)]
pub struct Harvest<'info> {
    #[account(mut)]
    pub farm: Account<'info, Farm>,
    #[account(mut, has_one = farm)]
    pub farm_log: Account<'info, FarmLog>,
    #[account(
        mut,
        has_one = farm,
        constraint = actor.tag != partner.tag @ ErrCode::CosplayBlocked
    )]
    pub actor: Account<'info, WorkerCard>,
    #[account(mut, has_one = farm)]
    pub partner: Account<'info, WorkerCard>,
    #[account(mut, has_one = farm)]
    pub silo: Account<'info, Silo>,
}

#[account]
pub struct Farm { pub owner: Pubkey, pub code: u32, pub harvests: u32 }

#[account]
pub struct WorkerCard { pub farm: Pubkey, pub kind: Worker, pub tag: u8, pub skill: u16 }

#[account]
pub struct Silo { pub farm: Pubkey, pub grain: u32 }

#[account]
pub struct FarmLog { pub farm: Pubkey, pub index: u32 }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Worker { Planter, Waterer, Harvester }

#[error_code]
pub enum ErrCode { #[msg("Type cosplay blocked in FarmTycoon.")] CosplayBlocked }
