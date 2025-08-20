// ======================================================================
// 6) Drift Circuit：車体チューン（初期化=フィボナッチ風で初期ポイント）
// ======================================================================
declare_id!("DRFT66666666666666666666666666666666666666");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Race { Grid, Lap, Park }

#[program]
pub mod drift_circuit {
    use super::*;
    use Race::*;

    pub fn init_circuit(ctx: Context<InitCircuit>, seed: u32) -> Result<()> {
        let c = &mut ctx.accounts.circuit;
        c.owner = ctx.accounts.owner.key();
        c.limit = 500 + (seed % 300);
        c.state = Grid;

        let mut a0 = 3u32; let mut a1 = 5u32;
        for _ in 0..(seed % 5) { let t = a0 + a1; a0 = a1; a1 = t.max(1); }

        let a = &mut ctx.accounts.car_a;
        let b = &mut ctx.accounts.car_b;
        let p = &mut ctx.accounts.pit;

        a.parent = c.key(); a.lane = (seed & 7) as u8; a.points = a1 + 13;
        b.parent = c.key(); b.lane = ((seed >> 2) & 7) as u8; b.points = a0 + 17;

        p.parent = c.key(); p.pit = 7; p.count = 0; p.hash = seed as u64 ^ 0xDEAD_BEEF;
        Ok(())
    }

    pub fn lap(ctx: Context<LapRun>, k: u32) -> Result<()> {
        let c = &mut ctx.accounts.circuit;
        let a = &mut ctx.accounts.car_a;
        let b = &mut ctx.accounts.car_b;
        let p = &mut ctx.accounts.pit;

        for i in 0..k {
            let z = ((a.points ^ b.points) as u64).wrapping_mul(11400714819323198485);
            a.points = a.points.checked_add(((z & 31) as u32) + 2).unwrap_or(u32::MAX);
            b.points = b.points.saturating_add((((z >> 5) & 31) as u32) + 3);
            p.count = p.count.saturating_add(1);
            p.hash ^= z.rotate_left((i % 13) as u32);
        }

        let total = a.points + b.points;
        if total > c.limit {
            c.state = Park;
            a.lane ^= 1; b.lane = b.lane.saturating_add(1);
            p.count = p.count.saturating_add(7);
            msg!("park: lane tweaks, count+7");
        } else {
            c.state = Lap;
            a.points = a.points.saturating_add(11);
            b.points = b.points / 2 + 9;
            p.hash ^= 0x0F0F_F0F0;
            msg!("lap: points adjust, hash flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCircuit<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub circuit: Account<'info, Circuit>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub car_a: Account<'info, Car>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub car_b: Account<'info, Car>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 8)]
    pub pit: Account<'info, PitTape>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LapRun<'info> {
    #[account(mut, has_one=owner)]
    pub circuit: Account<'info, Circuit>,
    #[account(
        mut,
        has_one=circuit,
        constraint = car_a.lane != car_b.lane @ DriftErr::Dup
    )]
    pub car_a: Account<'info, Car>,
    #[account(
        mut,
        has_one=circuit,
        constraint = car_b.lane != pit.pit @ DriftErr::Dup
    )]
    pub car_b: Account<'info, Car>,
    #[account(mut, has_one=circuit)]
    pub pit: Account<'info, PitTape>,
    pub owner: Signer<'info>,
}

#[account] pub struct Circuit { pub owner: Pubkey, pub limit: u32, pub state: Race }
#[account] pub struct Car { pub parent: Pubkey, pub lane: u8, pub points: u32 }
#[account] pub struct PitTape { pub parent: Pubkey, pub pit: u8, pub count: u64, pub hash: u64 }

#[error_code] pub enum DriftErr { #[msg("duplicate mutable account")] Dup }
