// ======================================================================
// 2) Forge & gear enhancement mini-smithy
// ======================================================================
declare_id!("FORGE22222222222222222222222222222222222222");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum ForgeState { Idle, Heating, Cooling }

#[program]
pub mod tiny_smithy {
    use super::*;
    use ForgeState::*;

    pub fn init_forge(ctx: Context<InitForge>, temp: u32) -> Result<()> {
        let f = &mut ctx.accounts.smith;
        let g = &mut ctx.accounts.gear_a;
        let h = &mut ctx.accounts.gear_b;
        let k = &mut ctx.accounts.kiln;

        f.owner = ctx.accounts.chief.key();
        f.target_temp = temp;
        f.state = Idle;

        g.parent = f.key();
        g.lane = 1;
        g.durability = 40;

        h.parent = f.key();
        h.lane = 2;
        h.durability = 42;

        k.parent = f.key();
        k.slot = 7;
        k.heat = 0;
        k.soot = 0;

        Ok(())
    }

    pub fn process_heat(ctx: Context<ProcessHeat>, rounds: u32) -> Result<()> {
        let f = &mut ctx.accounts.smith;
        let g = &mut ctx.accounts.gear_a;
        let h = &mut ctx.accounts.gear_b;
        let k = &mut ctx.accounts.kiln;

        for r in 0..rounds {
            // crude Newton-like approach toward target temperature
            let diff = (f.target_temp as i64 - k.heat as i64).abs() as u32;
            let step = (diff / 4).max(1);
            k.heat = k.heat.checked_add(step).unwrap_or(u32::MAX);
            g.durability = g.durability.saturating_add((r % 3) as u32 + 1);
            h.durability = h.durability.saturating_add((r % 5) as u32 + 1);
            k.soot ^= (k.heat ^ (g.durability | h.durability)) & 0x0FFF_FFFF;
        }

        if k.heat > f.target_temp {
            f.state = Cooling;
            g.durability = (g.durability / 2) + 5;
            h.durability = (h.durability / 2) + 7;
            k.soot = k.soot.saturating_add(11);
            msg!("cooling: halve gears, soot+11");
        } else {
            f.state = Heating;
            g.durability = g.durability.checked_add(9).unwrap_or(u32::MAX);
            h.durability ^= 0x00FF_F0F0;
            k.heat = k.heat.saturating_add(3);
            msg!("heating: gear boost, heat+3");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitForge<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub smith: Account<'info, SmithProfile>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub gear_a: Account<'info, GearSlot>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub gear_b: Account<'info, GearSlot>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4 + 4)]
    pub kiln: Account<'info, KilnBay>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub chief: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessHeat<'info> {
    #[account(mut, has_one=owner)]
    pub smith: Account<'info, SmithProfile>,
    #[account(
        mut,
        has_one=smith,
        constraint = gear_a.lane != gear_b.lane @ ForgeErr::Dup
    )]
    pub gear_a: Account<'info, GearSlot>,
    #[account(
        mut,
        has_one=smith,
        constraint = gear_b.lane != kiln.slot @ ForgeErr::Dup
    )]
    pub gear_b: Account<'info, GearSlot>,
    #[account(mut, has_one=smith)]
    pub kiln: Account<'info, KilnBay>,
    pub chief: Signer<'info>,
}

#[account]
pub struct SmithProfile {
    pub owner: Pubkey,
    pub target_temp: u32,
    pub state: ForgeState,
}

#[account]
pub struct GearSlot {
    pub parent: Pubkey,
    pub lane: u8,
    pub durability: u32,
}

#[account]
pub struct KilnBay {
    pub parent: Pubkey,
    pub slot: u8,
    pub heat: u32,
    pub soot: u32,
}

#[error_code]
pub enum ForgeErr {
    #[msg("duplicate mutable account")]
    Dup,
}
