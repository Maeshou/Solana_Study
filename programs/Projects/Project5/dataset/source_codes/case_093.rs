// ======================================================================
// 6) Aquifer Plant：揚水プラント（初期化＝モジュラ比率で初期流量）
// ======================================================================
declare_id!("WATR66666666666666666666666666666666666666");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum WaterState { Prime, Pump, Drain }

#[program]
pub mod aquifer_plant {
    use super::*;
    use WaterState::*;

    pub fn init_plant(ctx: Context<InitPlant>, minerals: u32) -> Result<()> {
        let p = &mut ctx.accounts.plant;
        p.owner = ctx.accounts.overseer.key();
        p.quota = (minerals as u64) * 5 + 1_000;
        p.state = Prime;

        let v1 = &mut ctx.accounts.valve_a;
        let v2 = &mut ctx.accounts.valve_b;
        let g = &mut ctx.accounts.gauge;

        // r = (minerals mod 97) / 97 を粗く使ったスケール
        let r = (minerals % 97 + 1) as u64;
        v1.plant = p.key(); v1.gate = (minerals & 7) as u8; v1.flow = ((r * 300) / 97) as u32 + 20;
        v2.plant = p.key(); v2.gate = ((minerals >> 2) & 7) as u8; v2.flow = ((r * 450) / 97) as u32 + 25;

        g.plant = p.key(); g.gate = 9; g.total = 0; g.salt = minerals ^ 0x1357_ABCD;
        Ok(())
    }

    pub fn run(ctx: Context<RunPump>, ticks: u32) -> Result<()> {
        let p = &mut ctx.accounts.plant;
        let a = &mut ctx.accounts.valve_a;
        let b = &mut ctx.accounts.valve_b;
        let g = &mut ctx.accounts.gauge;

        for i in 0..ticks {
            let mix = ((a.flow ^ b.flow) as u64).wrapping_mul(2654435761);
            a.flow = a.flow.checked_add(((mix & 31) as u32) + 2).unwrap_or(u32::MAX);
            b.flow = b.flow.saturating_add((((mix >> 5) & 31) as u32) + 3);
            g.total = g.total.saturating_add((a.flow as u64 + b.flow as u64) & 127);
            g.salt ^= (mix as u32).rotate_left((i % 13) as u32);
        }

        let sum = a.flow as u64 + b.flow as u64 + g.total;
        if sum > p.quota {
            p.state = Drain;
            a.gate ^= 1; b.gate = b.gate.saturating_add(1);
            g.gate = g.gate.saturating_add(1);
            msg!("drain: gate tweaks & gauge move");
        } else {
            p.state = Pump;
            a.flow = a.flow.saturating_add(9);
            b.flow = b.flow / 2 + 11;
            g.salt ^= 0x0F0F_F0F0;
            msg!("pump: flow adjust & salt flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPlant<'info> {
    #[account(init, payer=payer, space=8 + 32 + 8 + 1)]
    pub plant: Account<'info, Plant>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub valve_a: Account<'info, Valve>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub valve_b: Account<'info, Valve>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 4)]
    pub gauge: Account<'info, Gauge>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub overseer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RunPump<'info> {
    #[account(mut, has_one=owner)]
    pub plant: Account<'info, Plant>,
    #[account(
        mut,
        has_one=plant,
        constraint = valve_a.gate != valve_b.gate @ WaterErr::Dup
    )]
    pub valve_a: Account<'info, Valve>,
    #[account(
        mut,
        has_one=plant,
        constraint = valve_b.gate != gauge.gate @ WaterErr::Dup
    )]
    pub valve_b: Account<'info, Valve>,
    #[account(mut, has_one=plant)]
    pub gauge: Account<'info, Gauge>,
    pub overseer: Signer<'info>,
}

#[account] pub struct Plant { pub owner: Pubkey, pub quota: u64, pub state: WaterState }
#[account] pub struct Valve { pub plant: Pubkey, pub gate: u8, pub flow: u32 }
#[account] pub struct Gauge { pub plant: Pubkey, pub gate: u8, pub total: u64, pub salt: u32 }

#[error_code] pub enum WaterErr { #[msg("duplicate mutable account")] Dup }
