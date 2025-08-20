// ===============================================
// (7) energy_track: 進行度・エネルギー管理（プレイヤ・チケット・計測）
//   - 多層防御: has_one + ticket.slot 不一致
// ===============================================
use anchor_lang::prelude::*;
declare_id!("EnErGyTrAk777777777777777777777777777777");

#[program]
pub mod energy_track {
    use super::*;

    pub fn init_player(ctx: Context<InitPlayer>) -> Result<()> {
        let p = &mut ctx.accounts.player;
        p.owner = ctx.accounts.owner.key();
        p.energy = 100;
        p.level = 1;
        p.flags = 0;
        Ok(())
    }

    pub fn issue_ticket(ctx: Context<IssueTicket>, slot: u8, quota: u16) -> Result<()> {
        let t = &mut ctx.accounts.ticket;
        t.parent = ctx.accounts.player.key();
        t.slot = slot;
        t.quota = quota;
        t.used = 0;
        Ok(())
    }

    pub fn attach_meter(ctx: Context<AttachMeter>) -> Result<()> {
        let m = &mut ctx.accounts.meter;
        m.parent = ctx.accounts.player.key();
        m.consumed = 0;
        m.regen = 0;
        m.last = 0;
        Ok(())
    }

    /// 2枚のチケットの slot が一致していないことを要求（Type Cosplay 抑止）
    pub fn run_measure(ctx: Context<RunMeasure>, effort: u16) -> Result<()> {
        let p = &mut ctx.accounts.player;
        let ta = &mut ctx.accounts.ticket_a;
        let tb = &mut ctx.accounts.ticket_b;
        let m = &mut ctx.accounts.meter;

        // エネルギーの消費と再生（単純モデル）
        let consume = (effort as u64).min(p.energy as u64);
        p.energy = p.energy.saturating_sub(consume as u32);
        m.consumed = m.consumed.saturating_add(consume);

        // チケット消費（交互パス）
        if (consume & 1) == 0 {
            let step = ((consume as u16) % 7) as u16 + 1;
            ta.used = ta.used.saturating_add(step).min(ta.quota);
        } else {
            let step = ((consume as u16) % 5) as u16 + 1;
            tb.used = tb.used.saturating_add(step).min(tb.quota);
        }

        // 簡易レベルアップ／回復モデル
        if p.energy < 20 {
            let rec = (ta.slot as u32 + tb.slot as u32 + 5).min(25);
            p.energy = p.energy.saturating_add(rec);
            m.regen = m.regen.saturating_add(rec as u64);
        } else {
            p.level = p.level.saturating_add(1);
            m.last = (m.last.wrapping_add(consume as u32)) ^ ((ta.slot as u32) << 3);
        }
        Ok(())
    }
}

// -------------------- Accounts --------------------

#[derive(Accounts)]
pub struct InitPlayer<'info> {
    #[account(
        init,
        payer = owner,
        // 8 + 32(owner) + 4(energy) + 4(level) + 4(flags)
        space = 8 + 32 + 4 + 4 + 4
    )]
    pub player: Account<'info, Player>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IssueTicket<'info> {
    #[account(mut)]
    pub player: Account<'info, Player>,
    #[account(
        init,
        payer = owner,
        // 8 + 32(parent) + 1(slot) + 2(quota) + 2(used)
        space = 8 + 32 + 1 + 2 + 2
    )]
    pub ticket: Account<'info, Ticket>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AttachMeter<'info> {
    #[account(mut)]
    pub player: Account<'info, Player>,
    #[account(
        init,
        payer = owner,
        // 8 + 32(parent) + 8(consumed) + 8(regen) + 4(last)
        space = 8 + 32 + 8 + 8 + 4
    )]
    pub meter: Account<'info, Meter>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RunMeasure<'info> {
    #[account(mut)]
    pub player: Account<'info, Player>,
    #[account(
        mut,
        constraint = ticket_a.parent == player.key() @ TrackErr::Cosplay
    )]
    pub ticket_a: Account<'info, Ticket>,
    #[account(
        mut,
        constraint = ticket_b.parent == player.key() @ TrackErr::Cosplay,
        constraint = ticket_a.slot != ticket_b.slot @ TrackErr::Cosplay
    )]
    pub ticket_b: Account<'info, Ticket>,
    #[account(
        mut,
        constraint = meter.parent == player.key() @ TrackErr::Cosplay
    )]
    pub meter: Account<'info, Meter>,
}

// -------------------- Data --------------------

#[account]
pub struct Player {
    pub owner: Pubkey,
    pub energy: u32,
    pub level: u32,
    pub flags: u32,
}

#[account]
pub struct Ticket {
    pub parent: Pubkey, // = player
    pub slot: u8,
    pub quota: u16,
    pub used: u16,
}

#[account]
pub struct Meter {
    pub parent: Pubkey, // = player
    pub consumed: u64,
    pub regen: u64,
    pub last: u32,
}

#[error_code]
pub enum TrackErr { #[msg("cosplay blocked")] Cosplay }
