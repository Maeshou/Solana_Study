// ======================================================================
// 9) Space Dock：ドック割当（初期化=CRC風の初期チェック生成）
// ======================================================================
declare_id!("DOCK99999999999999999999999999999999999999");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum BayState { Queue, Service, Freeze }

#[program]
pub mod space_dock {
    use super::*;
    use BayState::*;

    pub fn init_dock(ctx: Context<InitDock>, n: u32) -> Result<()> {
        let d = &mut ctx.accounts.dock;
        d.owner = ctx.accounts.controller.key();
        d.maxq = n * 4 + 50;
        d.state = Queue;

        let a = &mut ctx.accounts.berth_a;
        let b = &mut ctx.accounts.berth_b;
        let q = &mut ctx.accounts.quay;

        // CRC風：poly=0x1EDC6F41 を雑に回す
        let mut crc = 0xFFFF_FFFFu32 ^ n;
        for _ in 0..4 { let bit = (crc ^ (n >> 5)) & 1; crc = (crc >> 1) ^ (if bit==1 {0x1EDC6F41} else {0}); }

        a.parent = d.key(); a.dock = (n & 7) as u8; a.load = (crc & 0x7FFF) + 20;
        b.parent = d.key(); b.dock = ((n >> 2) & 7) as u8; b.load = ((crc >> 7) & 0x7FFF) + 25;

        q.parent = d.key(); q.bay = 9; q.queue = 0; q.tag = crc as u64;
        Ok(())
    }

    pub fn service(ctx: Context<ServiceRun>, k: u32) -> Result<()> {
        let d = &mut ctx.accounts.dock;
        let a = &mut ctx.accounts.berth_a;
        let b = &mut ctx.accounts.berth_b;
        let q = &mut ctx.accounts.quay;

        for i in 0..k {
            let r = ((a.load ^ b.load) as u64).wrapping_mul(780291637);
            a.load = a.load.checked_add(((r & 63) as u32) + 4).unwrap_or(u32::MAX);
            b.load = b.load.saturating_add((((r >> 6) & 63) as u32) + 3);
            q.queue = q.queue.saturating_add((r & 31) as u64);
            q.tag ^= r.rotate_left((i % 23) as u32);
        }

        let sum = a.load + b.load;
        if sum > d.maxq {
            d.state = Freeze;
            a.dock ^= 1; b.dock = b.dock.saturating_add(1);
            q.bay = q.bay.saturating_add(1);
            msg!("freeze: bay++ & dock tweaks");
        } else {
            d.state = Service;
            a.load = a.load.saturating_add(9);
            b.load = b.load / 2 + 11;
            q.tag ^= 0x00FF_F0F0;
            msg!("service: adjust load & tag flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitDock<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub dock: Account<'info, Dock>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub berth_a: Account<'info, Berth>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub berth_b: Account<'info, Berth>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 8)]
    pub quay: Account<'info, Quay>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub controller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ServiceRun<'info> {
    #[account(mut, has_one=owner)]
    pub dock: Account<'info, Dock>,
    #[account(
        mut,
        has_one=dock,
        constraint = berth_a.dock != berth_b.dock @ DockErr::Dup
    )]
    pub berth_a: Account<'info, Berth>,
    #[account(
        mut,
        has_one=dock,
        constraint = berth_b.dock != quay.bay @ DockErr::Dup
    )]
    pub berth_b: Account<'info, Berth>,
    #[account(mut, has_one=dock)]
    pub quay: Account<'info, Quay>,
    pub controller: Signer<'info>,
}

#[account] pub struct Dock { pub owner: Pubkey, pub maxq: u32, pub state: BayState }
#[account] pub struct Berth { pub parent: Pubkey, pub dock: u8, pub load: u32 }
#[account] pub struct Quay { pub parent: Pubkey, pub bay: u8, pub queue: u64, pub tag: u64 }

#[error_code] pub enum DockErr { #[msg("duplicate mutable account")] Dup }
