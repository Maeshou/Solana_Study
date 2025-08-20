// ======================================================================
// 2) Mech Pit：整備ピット（初期化=LCGで初期ギア生成→親は最後に設定）
// ======================================================================
declare_id!("MECH22222222222222222222222222222222222222");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum BayMode { Idle, Tune, Lock }

#[program]
pub mod mech_pit {
    use super::*;
    use BayMode::*;

    pub fn init_bay(ctx: Context<InitBay>, seed: u64) -> Result<()> {
        let a = &mut ctx.accounts.module_a;
        let b = &mut ctx.accounts.module_b;
        let t = &mut ctx.accounts.tracker;
        // LCGで初期値を派生
        let mut s = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        a.slot = (s as u8) & 7; a.parent = ctx.accounts.bay.key(); a.torque = (s & 127) as u32 + 10;
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1);
        b.slot = (s as u8) & 7; b.parent = ctx.accounts.bay.key(); b.torque = ((s >> 7) & 255) as u32 + 12;

        t.parent = ctx.accounts.bay.key();
        t.ring = 9; t.counter = (seed as u32) ^ 0x1357_2468;

        // 親は“最後”にまとめて
        let bay = &mut ctx.accounts.bay;
        bay.owner = ctx.accounts.chief.key();
        bay.limit = ((seed as u32) & 1023) + 200;
        bay.mode = Idle;
        Ok(())
    }

    pub fn tune(ctx: Context<Tune>, rounds: u32) -> Result<()> {
        let bay = &mut ctx.accounts.bay;
        let a = &mut ctx.accounts.module_a;
        let b = &mut ctx.accounts.module_b;
        let t = &mut ctx.accounts.tracker;

        for i in 0..rounds {
            // 擬似PoW風：ハッシュカウンタが一定閾値を超えたらリセット
            t.counter = t.counter.rotate_left((i % 7) as u32).wrapping_add(0x9E37);
            a.torque = a.torque.checked_add((t.counter as u32 & 15) + 1).unwrap_or(u32::MAX);
            b.torque = b.torque.saturating_add(((t.counter >> 4) & 31) + 2);
        }

        let sum = a.torque + b.torque;
        if sum > bay.limit {
            bay.mode = Lock;
            a.torque = a.torque / 2 + 13;
            b.torque = b.torque / 2 + 11;
            t.ring ^= 0x3;
            msg!("lock: halves + ring flip");
        } else {
            bay.mode = Tune;
            a.slot = a.slot.saturating_add(1);
            b.slot ^= 0x1;
            t.counter ^= 0x00FF_F0F0;
            msg!("tune: slot shift + counter flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBay<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub bay: Account<'info, BayCore>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub module_a: Account<'info, Module>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub module_b: Account<'info, Module>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub tracker: Account<'info, Tracker>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub chief: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Tune<'info> {
    #[account(mut, has_one=owner)]
    pub bay: Account<'info, BayCore>,
    #[account(
        mut,
        has_one=bay,
        constraint = module_a.slot != module_b.slot @ MechErr::Dup
    )]
    pub module_a: Account<'info, Module>,
    #[account(
        mut,
        has_one=bay,
        constraint = module_b.slot != tracker.ring @ MechErr::Dup
    )]
    pub module_b: Account<'info, Module>,
    #[account(mut, has_one=bay)]
    pub tracker: Account<'info, Tracker>,
    pub chief: Signer<'info>,
}

#[account] pub struct BayCore { pub owner: Pubkey, pub limit: u32, pub mode: BayMode }
#[account] pub struct Module { pub parent: Pubkey, pub slot: u8, pub torque: u32 }
#[account] pub struct Tracker { pub parent: Pubkey, pub ring: u8, pub counter: u32 }

#[error_code] pub enum MechErr { #[msg("duplicate mutable account")] Dup }