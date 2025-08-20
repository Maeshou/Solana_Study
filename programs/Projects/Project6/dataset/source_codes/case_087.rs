// (2) Energy Forge — エネルギー炉とツール耐久管理
use anchor_lang::prelude::*;
declare_id!("EnErGyF0rGe2222222222222222222222222222222");

#[program]
pub mod energy_forge {
    use super::*;
    use ToolKind::*;

    pub fn init_station(ctx: Context<InitStation>, max_energy: u32) -> Result<()> {
        let st = &mut ctx.accounts.station;
        st.owner = ctx.accounts.admin.key();
        st.max_energy = max_energy;
        st.energy = max_energy / 2;
        st.calib = [0; 16];
        Ok(())
    }

    pub fn init_tool(ctx: Context<InitTool>, kind: ToolKind) -> Result<()> {
        let t = &mut ctx.accounts.tool;
        t.station = ctx.accounts.station.key();
        t.kind = kind;
        t.durability = 1000;
        t.usage_ring = [0; 12];
        Ok(())
    }

    pub fn process_cycle(ctx: Context<ProcessCycle>, cycles: u16) -> Result<()> {
        let st = &mut ctx.accounts.station;
        let a = &mut ctx.accounts.actor_tool;
        let b = &mut ctx.accounts.counter_tool;
        let log = &mut ctx.accounts.meter;

        let mut x = (st.energy as u64).max(1);
        let n = x;
        for _ in 0..6 { x = (x + n / x) >> 1; }
        let sqrt_e = (x as u32).min(st.max_energy);

        if a.kind == Hammer {
            a.durability = a.durability.saturating_sub((cycles as u32) + (sqrt_e & 0xFF));
            st.energy = st.energy.saturating_add((cycles as u32).min(50));
            log.pulses = log.pulses.saturating_add((sqrt_e / 3) as u64);
            log.flags = log.flags | 0b01;
            msg!("Hammer path: durability down, energy up, meter flags set");
        } else {
            b.durability = b.durability.saturating_sub((cycles as u32).saturating_mul(2));
            st.energy = st.energy.saturating_sub((cycles as u32).min(st.energy));
            log.pulses = log.pulses ^ ((sqrt_e as u64).rotate_left(5));
            log.flags = log.flags | 0b10;
            msg!("Non-Hammer path: counter tool used and energy consumed");
        }

        for i in 0..st.calib.len() {
            let inc = ((i as u32) ^ (cycles as u32)).count_ones() as u32;
            st.calib[i] = st.calib[i].saturating_add(inc & 0xF);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStation<'info> {
    #[account(init, payer = admin, space = 8 + Station::MAX)]
    pub station: Account<'info, Station>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct InitTool<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub station: Account<'info, Station>,
    #[account(init, payer = user, space = 8 + Tool::MAX)]
    pub tool: Account<'info, Tool>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ProcessCycle<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub station: Account<'info, Station>,
    #[account(mut, has_one = station, owner = crate::ID)]
    pub actor_tool: Account<'info, Tool>,
    #[account(
        mut,
        has_one = station,
        owner = crate::ID,
        constraint = actor_tool.kind != counter_tool.kind @ ErrCode::CosplayBlocked
    )]
    pub counter_tool: Account<'info, Tool>,
    #[account(mut, has_one = station, owner = crate::ID)]
    pub meter: Account<'info, Meter>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Station { pub owner: Pubkey, pub max_energy: u32, pub energy: u32, pub calib: [u32; 16] }
impl Station { pub const MAX: usize = 32 + 4 + 4 + 4 * 16; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum ToolKind { Hammer, Wrench, Drill }
use ToolKind::*;

#[account]
pub struct Tool { pub station: Pubkey, pub kind: ToolKind, pub durability: u32, pub usage_ring: [u32; 12] }
impl Tool { pub const MAX: usize = 32 + 1 + 4 + 4 * 12; }

#[account]
pub struct Meter { pub station: Pubkey, pub pulses: u64, pub flags: u32 }
impl Meter { pub const MAX: usize = 32 + 8 + 4; }

#[error_code]
pub enum ErrCode { #[msg("Type Cosplay blocked by kind mismatch")] CosplayBlocked }
