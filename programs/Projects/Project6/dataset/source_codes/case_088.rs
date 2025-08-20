// (3) Raid Ledger — レイド記録台帳とパーティスロット
use anchor_lang::prelude::*;
declare_id!("Ra1dLedg3r3333333333333333333333333333333");

#[program]
pub mod raid_ledger {
    use super::*;
    use SlotTag::*;

    pub fn init_raid(ctx: Context<InitRaid>, boss: String) -> Result<()> {
        let r = &mut ctx.accounts.raid;
        r.owner = ctx.accounts.host.key();
        r.boss = boss;
        r.total_round = 0;
        Ok(())
    }

    pub fn init_slot(ctx: Context<InitSlot>, tag: SlotTag) -> Result<()> {
        let s = &mut ctx.accounts.slot;
        s.raid = ctx.accounts.raid.key();
        s.tag = tag;
        s.total_damage = 0;
        s.buff = 0;
        Ok(())
    }

    pub fn record_round(ctx: Context<RecordRound>, seed: u64) -> Result<()> {
        let raid = &mut ctx.accounts.raid;
        let a = &mut ctx.accounts.attacker;
        let d = &mut ctx.accounts.defender;
        let book = &mut ctx.accounts.book;

        let mut rolling = seed;
        for _ in 0..8 {
            rolling = rolling.rotate_left(3) ^ 0x9E3779B97F4A7C15u64;
            a.buff = a.buff ^ ((rolling as u32) & 0x3F);
            book.hash = book.hash.wrapping_add(rolling);
        }

        if (rolling & 1) == 0 {
            a.total_damage = a.total_damage.saturating_add(((rolling >> 8) & 0xFFFF) as u64);
            raid.total_round = raid.total_round.saturating_add(1);
            book.lines = book.lines.saturating_add(1);
            book.last = rolling;
            msg!("Even roll: attacker scores and round increments");
        } else {
            d.total_damage = d.total_damage.saturating_add(((rolling >> 12) & 0x7FFF) as u64);
            raid.total_round = raid.total_round.saturating_add(1);
            book.lines = book.lines.saturating_add(2);
            book.last = rolling ^ book.hash;
            msg!("Odd roll: defender absorbs and record adjusted");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRaid<'info> {
    #[account(init, payer = host, space = 8 + Raid::MAX)]
    pub raid: Account<'info, Raid>,
    #[account(mut)]
    pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct InitSlot<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub raid: Account<'info, Raid>,
    #[account(init, payer = user, space = 8 + Slot::MAX)]
    pub slot: Account<'info, Slot>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct RecordRound<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub raid: Account<'info, Raid>,
    #[account(mut, has_one = raid, owner = crate::ID)]
    pub book: Account<'info, Book>,
    #[account(mut, has_one = raid, owner = crate::ID)]
    pub attacker: Account<'info, Slot>,
    #[account(
        mut,
        has_one = raid,
        owner = crate::ID,
        constraint = attacker.tag != defender.tag @ ErrCode::CosplayBlocked
    )]
    pub defender: Account<'info, Slot>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Raid { pub owner: Pubkey, pub boss: String, pub total_round: u64 }
impl Raid { pub const MAX: usize = 32 + 4 + 64 + 8; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum SlotTag { Tank, Healer, DPS }
use SlotTag::*;

#[account]
pub struct Slot { pub raid: Pubkey, pub tag: SlotTag, pub total_damage: u64, pub buff: u32 }
impl Slot { pub const MAX: usize = 32 + 1 + 8 + 4; }

#[account]
pub struct Book { pub raid: Pubkey, pub hash: u64, pub last: u64, pub lines: u32 }
impl Book { pub const MAX: usize = 32 + 8 + 8 + 4; }

#[error_code]
pub enum ErrCode { #[msg("Type Cosplay blocked by slot tag mismatch")] CosplayBlocked }
