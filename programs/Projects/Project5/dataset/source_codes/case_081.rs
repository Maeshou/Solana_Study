// ======================================================================
// 4) Tea House：茶房（初期化＝ビットマスクで初期温度/香り組立）
// ======================================================================
declare_id!("TEAH44444444444444444444444444444444444444");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum BrewState { Warm, Steep, Rest }

#[program]
pub mod teahouse_brew {
    use super::*;
    use BrewState::*;

    pub fn init_teahouse(ctx: Context<InitTeahouse>, mask: u32) -> Result<()> {
        let h = &mut ctx.accounts.house;
        h.owner = ctx.accounts.host.key();
        h.target = 70 + (mask & 31);
        h.state = Warm;

        let k = &mut ctx.accounts.kettle_a;
        let l = &mut ctx.accounts.kettle_b;
        let t = &mut ctx.accounts.taster;

        k.house = h.key(); k.spout = (mask & 7) as u8; k.heat = (mask % 90) + 12;
        l.house = h.key(); l.spout = ((mask >> 4) & 7) as u8; l.heat = ((mask >> 2) % 90) + 11;

        t.house = h.key(); t.spout = 9; t.ticks = 0; t.check = (mask as u64) ^ 0xAA55_AA55;
        Ok(())
    }

    pub fn steep(ctx: Context<Steep>, cycles: u32) -> Result<()> {
        let h = &mut ctx.accounts.house;
        let a = &mut ctx.accounts.kettle_a;
        let b = &mut ctx.accounts.kettle_b;
        let t = &mut ctx.accounts.taster;

        for i in 0..cycles {
            // 三角波×EMA風
            let tri = ((i % 10) as i32 - 5).abs() as u32 + 1;
            a.heat = ((a.heat as u64 * 7 + (tri + 2) as u64 * 3) / 10) as u32;
            b.heat = b.heat.saturating_add(tri / 2 + 1);
            t.ticks = t.ticks.saturating_add(1);
            t.check ^= ((a.heat ^ b.heat) as u64) << (i % 8);
        }

        let avg = (a.heat + b.heat) / 2;
        if avg > h.target {
            h.state = Rest;
            a.spout ^= 1;
            b.spout = b.spout.saturating_add(1);
            t.spout = t.spout.saturating_add(1);
            msg!("rest: spout tweak & taster move");
        } else {
            h.state = Steep;
            a.heat = a.heat.saturating_add(7);
            b.heat = b.heat / 2 + 9;
            t.check ^= 0x0F0F_F0F0;
            msg!("steep: adjust heat & check flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTeahouse<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub house: Account<'info, TeaHouse>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub kettle_a: Account<'info, Kettle>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub kettle_b: Account<'info, Kettle>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 8)]
    pub taster: Account<'info, TasterLog>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Steep<'info> {
    #[account(mut, has_one=owner)]
    pub house: Account<'info, TeaHouse>,
    #[account(
        mut,
        has_one=house,
        constraint = kettle_a.spout != kettle_b.spout @ TeaErr::Dup
    )]
    pub kettle_a: Account<'info, Kettle>,
    #[account(
        mut,
        has_one=house,
        constraint = kettle_b.spout != taster.spout @ TeaErr::Dup
    )]
    pub kettle_b: Account<'info, Kettle>,
    #[account(mut, has_one=house)]
    pub taster: Account<'info, TasterLog>,
    pub host: Signer<'info>,
}

#[account] pub struct TeaHouse { pub owner: Pubkey, pub target: u32, pub state: BrewState }
#[account] pub struct Kettle   { pub house: Pubkey, pub spout: u8, pub heat: u32 }
#[account] pub struct TasterLog{ pub house: Pubkey, pub spout: u8, pub ticks: u64, pub check: u64 }

#[error_code] pub enum TeaErr { #[msg("duplicate mutable account")] Dup }
