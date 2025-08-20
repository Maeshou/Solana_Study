// ======================================================================
// 4) Galley Kitchen：調理ステーション（初期化=ビットマスクで初期フラグ合成）
// ======================================================================
declare_id!("COOK44444444444444444444444444444444444444");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Service { Prep, Serve, Close }

#[program]
pub mod galley_kitchen {
    use super::*;
    use Service::*;

    pub fn init_line(ctx: Context<InitLine>, mask: u32) -> Result<()> {
        let k = &mut ctx.accounts.kitchen;
        k.owner = ctx.accounts.chef.key();
        k.flag = (mask & 0x0FFF) | ((mask.rotate_left(5)) & 0xF000);
        k.state = Prep;

        let s1 = &mut ctx.accounts.station_a;
        let s2 = &mut ctx.accounts.station_b;
        let tk = &mut ctx.accounts.ticket;

        s1.parent = k.key(); s1.station = (mask & 7) as u8; s1.heat = (mask % 90) + 10;
        s2.parent = k.key(); s2.station = ((mask >> 4) & 7) as u8; s2.heat = ((mask >> 2) % 90) + 12;

        tk.parent = k.key(); tk.number = 9; tk.queue = (mask as u64) & 0xFFFF; tk.mix = 0;
        Ok(())
    }

    pub fn fire(ctx: Context<Fire>, steps: u32) -> Result<()> {
        let k = &mut ctx.accounts.kitchen;
        let a = &mut ctx.accounts.station_a;
        let b = &mut ctx.accounts.station_b;
        let t = &mut ctx.accounts.ticket;

        for i in 0..steps {
            let tri = ((i % 10) as i32 - 5).abs() as u32; // 三角波っぽい
            a.heat = a.heat.checked_add(tri + 1).unwrap_or(u32::MAX);
            b.heat = b.heat.saturating_add((tri / 2) + 1);
            t.mix ^= ((a.heat ^ b.heat) as u64) << (i % 8);
        }

        let avg = ((a.heat + b.heat) / 2) as u32;
        if avg > (k.flag & 0x0FFF) {
            k.state = Close;
            a.station ^= 0x1;
            b.station = b.station.saturating_add(1);
            t.queue = t.queue.saturating_add((avg as u64) & 63);
            msg!("close: station flip & queue+");
        } else {
            k.state = Serve;
            k.flag ^= 0x00FF;
            a.heat = a.heat.saturating_add(7);
            b.heat = b.heat / 2 + 9;
            msg!("serve: heat adjust & flag flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLine<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub kitchen: Account<'info, Kitchen>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub station_a: Account<'info, Station>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub station_b: Account<'info, Station>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 8)]
    pub ticket: Account<'info, Ticket>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub chef: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Fire<'info> {
    #[account(mut, has_one=owner)]
    pub kitchen: Account<'info, Kitchen>,
    #[account(
        mut,
        has_one=kitchen,
        constraint = station_a.station != station_b.station @ CookErr::Dup
    )]
    pub station_a: Account<'info, Station>,
    #[account(
        mut,
        has_one=kitchen,
        constraint = station_b.station != ticket.number @ CookErr::Dup
    )]
    pub station_b: Account<'info, Station>,
    #[account(mut, has_one=kitchen)]
    pub ticket: Account<'info, Ticket>,
    pub chef: Signer<'info>,
}

#[account] pub struct Kitchen { pub owner: Pubkey, pub flag: u32, pub state: Service }
#[account] pub struct Station { pub parent: Pubkey, pub station: u8, pub heat: u32 }
#[account] pub struct Ticket { pub parent: Pubkey, pub number: u8, pub queue: u64, pub mix: u64 }

#[error_code] pub enum CookErr { #[msg("duplicate mutable account")] Dup }
