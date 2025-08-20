// ======================================================================
// 3) Raid recorder: party vs boss notes
// ======================================================================
declare_id!("RAID333333333333333333333333333333333333333");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum RaidMode { Prep, Assault, Recover }

#[program]
pub mod raid_recorder {
    use super::*;
    use RaidMode::*;

    pub fn init_raid(ctx: Context<InitRaid>, weight: u64) -> Result<()> {
        let p = &mut ctx.accounts.party;
        let b = &mut ctx.accounts.boss_note;
        let l = &mut ctx.accounts.battle_log;

        p.owner = ctx.accounts.leader.key();
        p.weight = weight;
        p.mode = Prep;

        b.parent = p.key();
        b.phase = 1;
        b.hp = 1000;

        l.parent = p.key();
        l.phase = 9;
        l.ticks = 0;
        l.acc = 0;

        Ok(())
    }

    pub fn write_tick(ctx: Context<WriteTick>, loops: u32) -> Result<()> {
        let p = &mut ctx.accounts.party;
        let b = &mut ctx.accounts.boss_note;
        let l = &mut ctx.accounts.battle_log;

        for i in 0..loops {
            // integer sqrt via Newton step on boss hp
            let mut x = (b.hp as u64).max(1);
            let mut y = (x + 1) / 2;
            while y < x { x = y; y = (x + (b.hp as u64 / x)) / 2; }
            let root = x as u32;

            b.hp = b.hp.saturating_sub((i % 7) as u32 + (root % 5));
            l.ticks = l.ticks.saturating_add(1);
            l.acc = l.acc.checked_add((b.hp as u64) & 0xFFFF).unwrap_or(u64::MAX);
        }

        let avg = if l.ticks == 0 { 0 } else { (l.acc / l.ticks as u64) as u32 };
        if avg > p.weight as u32 {
            p.mode = Recover;
            b.hp = b.hp.saturating_add(33);
            l.phase = l.phase.saturating_add(1);
            l.acc ^= 0x0FFF_FFFF;
            msg!("recover: boss heal, phase+1");
        } else {
            p.mode = Assault;
            b.phase = b.phase.saturating_add(1);
            l.ticks = l.ticks.saturating_add(3);
            l.acc = l.acc.saturating_add((avg as u64) << 2);
            msg!("assault: phase++ ticks+3");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRaid<'info> {
    #[account(init, payer=payer, space=8 + 32 + 8 + 1)]
    pub party: Account<'info, Party>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub boss_note: Account<'info, BossNote>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 8)]
    pub battle_log: Account<'info, BattleLog>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WriteTick<'info> {
    #[account(mut, has_one=owner)]
    pub party: Account<'info, Party>,
    #[account(
        mut,
        has_one=party,
        constraint = boss_note.phase != battle_log.phase @ RaidErr::Dup
    )]
    pub boss_note: Account<'info, BossNote>,
    #[account(mut, has_one=party)]
    pub battle_log: Account<'info, BattleLog>,
    pub leader: Signer<'info>,
}

#[account]
pub struct Party {
    pub owner: Pubkey,
    pub weight: u64,
    pub mode: RaidMode,
}

#[account]
pub struct BossNote {
    pub parent: Pubkey,
    pub phase: u8,
    pub hp: u32,
}

#[account]
pub struct BattleLog {
    pub parent: Pubkey,
    pub phase: u8,
    pub ticks: u64,
    pub acc: u64,
}

#[error_code]
pub enum RaidErr {
    #[msg("duplicate mutable account")]
    Dup,
}
