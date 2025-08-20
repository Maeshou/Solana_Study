// ======================================================================
// 10) Pigeon Loft：鳩舎（初期化＝16bit LFSRで初期体力、順：親→台帳→鳥A→鳥B）
// ======================================================================
declare_id!("PGON10101010101010101010101010101010101010");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum LoftStage { Nest, Flight, Roost }

#[program]
pub mod pigeon_loft {
    use super::*;
    use LoftStage::*;

    pub fn init_loft(ctx: Context<InitLoft>, salt: u16) -> Result<()> {
        let lf = &mut ctx.accounts.loft;
        lf.owner = ctx.accounts.keeper.key();
        lf.cap = (salt as u32) * 6 + 300;
        lf.stage = Nest;

        let lg = &mut ctx.accounts.ledger;
        lg.loft = lf.key(); lg.perch = 9; lg.hops = 0; lg.mix = salt as u64 ^ 0xAA55;

        // 16bit LFSR (taps: 16,14,13,11)
        let mut s = salt as u32;
        let step = |s: &mut u32| {
            let b = ((*s >> 0) ^ (*s >> 2) ^ (*s >> 3) ^ (*s >> 5)) & 1;
            *s = (*s >> 1) | (b << 15);
            *s
        };

        let a = &mut ctx.accounts.bird_a;
        a.loft = lf.key(); a.perch = (s as u8) & 7; a.stamina = (step(&mut s) & 511) + 50;

        let b = &mut ctx.accounts.bird_b;
        b.loft = lf.key(); b.perch = ((s >> 3) as u8) & 7; b.stamina = (step(&mut s) & 511) + 45;

        Ok(())
    }

    pub fn fly(ctx: Context<FlyOut>, laps: u32) -> Result<()> {
        let lf = &mut ctx.accounts.loft;
        let a = &mut ctx.accounts.bird_a;
        let b = &mut ctx.accounts.bird_b;
        let lg = &mut ctx.accounts.ledger;

        for i in 0..laps {
            let mix = ((a.stamina ^ b.stamina) as u64).wrapping_mul(2654435761);
            a.stamina = a.stamina.checked_add(((mix & 31) as u32) + 2).unwrap_or(u32::MAX);
            b.stamina = b.stamina.saturating_add((((mix >> 5) & 31) as u32) + 3);
            lg.hops = lg.hops.saturating_add(1);
            lg.mix ^= mix.rotate_left((i % 13) as u32);
        }

        let sum = a.stamina + b.stamina + (lg.hops as u32 & 0x1FF);
        if sum > lf.cap {
            lf.stage = Roost;
            a.perch ^= 1; b.perch = b.perch.saturating_add(1);
            lg.perch = lg.perch.saturating_add(1);
            msg!("roost: perch tweaks & ledger move");
        } else {
            lf.stage = Flight;
            a.stamina = a.stamina.saturating_add(9);
            b.stamina = b.stamina / 2 + 11;
            lg.mix ^= 0x0F0F_F0F0;
            msg!("flight: stamina adjust & mix flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLoft<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub loft: Account<'info, Loft>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 8)]
    pub ledger: Account<'info, LoftLog>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub bird_a: Account<'info, Bird>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub bird_b: Account<'info, Bird>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub keeper: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FlyOut<'info> {
    #[account(mut, has_one=owner)]
    pub loft: Account<'info, Loft>,
    #[account(
        mut,
        has_one=loft,
        constraint = bird_a.perch != bird_b.perch @ LoftErr::Dup
    )]
    pub bird_a: Account<'info, Bird>,
    #[account(
        mut,
        has_one=loft,
        constraint = bird_b.perch != ledger.perch @ LoftErr::Dup
    )]
    pub bird_b: Account<'info, Bird>,
    #[account(mut, has_one=loft)]
    pub ledger: Account<'info, LoftLog>,
    pub keeper: Signer<'info>,
}

#[account] pub struct Loft { pub owner: Pubkey, pub cap: u32, pub stage: LoftStage }
#[account] pub struct Bird { pub loft: Pubkey, pub perch: u8, pub stamina: u32 }
#[account] pub struct LoftLog { pub loft: Pubkey, pub perch: u8, pub hops: u64, pub mix: u64 }

#[error_code] pub enum LoftErr { #[msg("duplicate mutable account")] Dup }






