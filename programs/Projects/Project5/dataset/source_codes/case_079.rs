// ======================================================================
// 2) Courier Hub：荷さばき（初期化＝LCG計算→ベルト/ビンへ分配）
// ======================================================================
declare_id!("HUB222222222222222222222222222222222222222");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum HubState { Idle, Routing, Blocked }

#[program]
pub mod courier_hub {
    use super::*;
    use HubState::*;

    pub fn init_hub(ctx: Context<InitHub>, seed: u64) -> Result<()> {
        let belt_a = &mut ctx.accounts.belt_a;
        let belt_b = &mut ctx.accounts.belt_b;
        let bin = &mut ctx.accounts.bin;
        let hub = &mut ctx.accounts.hub;

        // LCG
        let mut s = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        belt_a.hub = hub.key(); belt_a.lane = (s as u8) & 7; belt_a.parcels = (s & 511) as u32 + 40;
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1);
        belt_b.hub = hub.key(); belt_b.lane = (s as u8) & 7; belt_b.parcels = ((s >> 9) & 511) as u32 + 35;

        bin.hub = hub.key(); bin.lane = 9; bin.load = 0; bin.tag = (seed as u8) ^ 0x5A;

        hub.owner = ctx.accounts.operator.key();
        hub.capacity = ((seed as u32) & 0x0FFF) + 600;
        hub.state = Idle;
        Ok(())
    }

    pub fn route(ctx: Context<Route>, t: u32) -> Result<()> {
        let hub = &mut ctx.accounts.hub;
        let a = &mut ctx.accounts.belt_a;
        let b = &mut ctx.accounts.belt_b;
        let bin = &mut ctx.accounts.bin;

        for i in 0..t {
            let mix = ((a.parcels ^ b.parcels) as u64).wrapping_mul(780291637);
            a.parcels = a.parcels.checked_add(((mix & 31) as u32) + 2).unwrap_or(u32::MAX);
            b.parcels = b.parcels.saturating_add((((mix >> 5) & 31) as u32) + 1);
            bin.load = bin.load.saturating_add((mix & 15) as u64);
            bin.tag ^= (mix as u8).rotate_left((i % 5) as u32);
        }

        let sum = a.parcels + b.parcels;
        if sum > hub.capacity {
            hub.state = Blocked;
            bin.lane = bin.lane.saturating_add(1);
            a.lane ^= 1;
            b.lane = b.lane.saturating_add(1);
            msg!("blocked: lane tweaks & bin shift");
        } else {
            hub.state = Routing;
            a.parcels = a.parcels.saturating_add(11);
            b.parcels = b.parcels / 2 + 9;
            bin.tag ^= 0xF0;
            msg!("routing: adjust parcels & tag flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitHub<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub hub: Account<'info, HubCore>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub belt_a: Account<'info, Belt>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub belt_b: Account<'info, Belt>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 1)]
    pub bin: Account<'info, BinStore>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Route<'info> {
    #[account(mut, has_one=owner)]
    pub hub: Account<'info, HubCore>,
    #[account(
        mut,
        has_one=hub,
        constraint = belt_a.lane != belt_b.lane @ HubErr::Dup
    )]
    pub belt_a: Account<'info, Belt>,
    #[account(
        mut,
        has_one=hub,
        constraint = belt_b.lane != bin.lane @ HubErr::Dup
    )]
    pub belt_b: Account<'info, Belt>,
    #[account(mut, has_one=hub)]
    pub bin: Account<'info, BinStore>,
    pub operator: Signer<'info>,
}

#[account] pub struct HubCore { pub owner: Pubkey, pub capacity: u32, pub state: HubState }
#[account] pub struct Belt    { pub hub: Pubkey, pub lane: u8, pub parcels: u32 }
#[account] pub struct BinStore{ pub hub: Pubkey, pub lane: u8, pub load: u64, pub tag: u8 }

#[error_code] pub enum HubErr { #[msg("duplicate mutable account")] Dup }
