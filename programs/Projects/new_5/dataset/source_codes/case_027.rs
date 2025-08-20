// ============================================================================
// 3) Raid Chronicle (two mutable logs)
// ============================================================================
use anchor_lang::prelude::*;

declare_id!("RAID3333333333333333333333333333333333333");

#[program]
pub mod raid_chronicle {
    use super::*;
    use BossState::*;

    pub fn init_party(ctx: Context<InitParty>, flag: u8) -> Result<()> {
        let p = &mut ctx.accounts.party;
        p.leader = ctx.accounts.leader.key();
        p.flag = flag;
        p.power = 100;
        p.raid_count = 0;
        Ok(())
    }

    pub fn init_log(ctx: Context<InitLog>, boss_id: u32) -> Result<()> {
        let l = &mut ctx.accounts.raid_log;
        l.parent = ctx.accounts.party.key();
        l.boss_id = boss_id;
        l.state = Dormant;
        l.damage = 0;
        l.turns = 0;
        Ok(())
    }

    pub fn record_rounds(ctx: Context<RecordRounds>, bursts: u32) -> Result<()> {
        let p = &mut ctx.accounts.party;
        let a = &mut ctx.accounts.log_a;
        let b = &mut ctx.accounts.log_b;

        // moving average style loop
        let mut avg: u64 = p.power as u64;
        for k in 0..6 {
            let hit = ((bursts as u64 + k as u64 * 17) % 300) as u64;
            avg = (avg * 3 + hit) / 4;
            p.raid_count = p.raid_count.saturating_add(1);
        }

        if avg as u32 > p.power {
            a.state = Raging;
            a.damage = a.damage.saturating_add((avg as u32) / 3 + 10);
            a.turns = a.turns.saturating_add(2);
            p.power = (p.power + ((avg as u32) & 255)).min(10_000);
            msg!("A raging; dmg={}, power={}", a.damage, p.power);
        } else {
            a.state = Dormant;
            a.damage = a.damage / 2 + (bursts & 31);
            a.turns = a.turns.saturating_sub(1);
            p.power = (p.power / 2) + 77;
            msg!("A calmed; dmg={}, power={}", a.damage, p.power);
        }

        for _ in 0..4 {
            if (b.boss_id ^ (p.power as u32)) & 1 == 1 {
                b.state = Enraged;
                b.damage = b.damage.saturating_add(bursts / 2 + 5);
                p.power = p.power.saturating_add(9);
                msg!("B enraged; dmg={}, power={}", b.damage, p.power);
            } else {
                b.state = Dormant;
                b.turns = b.turns.saturating_add(1);
                p.power = p.power.saturating_sub(3);
                msg!("B stall; turns={}, power={}", b.turns, p.power);
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitParty<'info> {
    #[account(init, payer = leader, space = 8 + 32 + 1 + 4 + 8)]
    pub party: Account<'info, Party>,
    #[account(mut)]
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitLog<'info> {
    #[account(mut)]
    pub party: Account<'info, Party>,
    #[account(init, payer = writer, space = 8 + 32 + 4 + 1 + 4 + 4)]
    pub raid_log: Account<'info, RaidLog>,
    #[account(mut)]
    pub writer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordRounds<'info> {
    #[account(mut)]
    pub party: Account<'info, Party>,
    #[account(mut, has_one = parent)]
    pub log_a: Account<'info, RaidLog>,
    #[account(mut, has_one = parent)]
    pub log_b: Account<'info, RaidLog>, // can alias
}

#[account]
pub struct Party {
    pub leader: Pubkey,
    pub flag: u8,
    pub power: u32,
    pub raid_count: u64,
}

#[account]
pub struct RaidLog {
    pub parent: Pubkey,
    pub boss_id: u32,
    pub state: BossState,
    pub damage: u32,
    pub turns: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum BossState {
    Dormant,
    Raging,
    Enraged,
}
use BossState::*;

#[error_code]
pub enum RaidError {
    #[msg("raid error")]
    RaidErrorGeneric,
}
