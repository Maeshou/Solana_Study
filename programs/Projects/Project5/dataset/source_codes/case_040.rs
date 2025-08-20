// ============================================================================
// 9) Comet Farm — Fibonacci風/マスク/折り返しクリップ — PDAあり
// ============================================================================
declare_id!("CMFM999999999999999999999999999999999");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum FarmState { Start, Grow, Cap }

#[program]
pub mod comet_farm {
    use super::*;

    pub fn init_farm(ctx: Context<InitFarm>, cap: u64) -> Result<()> {
        let f = &mut ctx.accounts;
        f.cfg.owner = f.owner.key();
        f.cfg.cap = cap;
        f.cfg.state = FarmState::Start;
        Ok(())
    }

    pub fn cultivate(ctx: Context<Cultivate>, ticks: u32) -> Result<()> {
        let f = &mut ctx.accounts;
        assert_ne!(f.cfg.key(), f.log.key(), "cfg/log must differ");

        for _ in 0..ticks {
            // F(n+1)=F(n)+F(n-1) 風。ただし 40bit で折り返し
            let next = (u128::from(f.field.a) + u128::from(f.field.b)) & ((1u128<<40)-1);
            f.field.a = f.field.b;
            f.field.b = next as u64;

            // logにはハミング重み風の加点（popcount近似：x^(x>>1)の下位)
            let x = f.field.b as u32;
            let v = x ^ (x >> 1) ^ (x >> 2);
            f.log.events = f.log.events.wrapping_add((v & 0xFF) as u32);
        }

        if f.field.b > f.cfg.cap {
            f.cfg.state = FarmState::Cap;
            f.field.b = f.cfg.cap; // クリップ
            f.log.badges = f.log.badges.wrapping_add(2);
            msg!("cap reached: clip b to cap, badges+2");
        } else {
            f.cfg.state = FarmState::Grow;
            f.field.a = f.field.a + (f.field.b / 3);
            f.log.events = f.log.events.wrapping_add(1);
            msg!("grow: a += b/3, events+1");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitFarm<'info> {
    #[account(init, payer=payer, space=8+32+8+1, seeds=[b"cfg", owner.key().as_ref()], bump)]
    pub cfg: Account<'info, FarmCfg>,
    #[account(init, payer=payer, space=8+8+8)]
    pub field: Account<'info, FieldPair>,
    #[account(init, payer=payer, space=8+4+4)]
    pub log: Account<'info, FarmLog>,
    #[account(mut)] pub payer: Signer<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Cultivate<'info> {
    #[account(mut, seeds=[b"cfg", owner.key().as_ref()], bump, has_one=owner)]
    pub cfg: Account<'info, FarmCfg>,
    #[account(mut, constraint = cfg.key() != field.key(), error = FarmErr::Dup)]
    pub field: Account<'info, FieldPair>,
    #[account(mut)]
    pub log: Account<'info, FarmLog>,
    pub owner: Signer<'info>,
}

#[account] pub struct FarmCfg { pub owner: Pubkey, pub cap: u64, pub state: FarmState }
#[account] pub struct FieldPair { pub a: u64, pub b: u64 }
#[account] pub struct FarmLog { pub events: u32, pub badges: u32 }

#[error_code] pub enum FarmErr { #[msg("dup")] Dup }