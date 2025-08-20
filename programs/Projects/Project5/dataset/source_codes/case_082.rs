// ======================================================================
// 5) Star Garden：観測（初期化＝ユークリッド近似距離で焦点値を設定）
// ======================================================================
declare_id!("STAR55555555555555555555555555555555555555");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum SkyMode { Setup, Observe, Sleep }

#[program]
pub mod star_garden {
    use super::*;
    use SkyMode::*;

    pub fn init_observatory(ctx: Context<InitObs>, x: u32, y: u32) -> Result<()> {
        let o = &mut ctx.accounts.obs;
        o.owner = ctx.accounts.astronomer.key();
        o.limit = x + y + 300;
        o.mode = Setup;

        let a = &mut ctx.accounts.scope_a;
        let b = &mut ctx.accounts.scope_b;
        let lg = &mut ctx.accounts.logbook;

        let r = x.max(y) + y.min(x) / 2;
        a.obs = o.key(); a.slot = (x & 7) as u8; a.focus = r + 7;
        b.obs = o.key(); b.slot = (y & 7) as u8; b.focus = r.rotate_left(3) + 9;

        lg.obs = o.key(); lg.slot = 9; lg.count = 0; lg.mix = (x as u64) << 10 | (y as u64);
        Ok(())
    }

    pub fn watch(ctx: Context<Watch>, n: u32) -> Result<()> {
        let o = &mut ctx.accounts.obs;
        let a = &mut ctx.accounts.scope_a;
        let b = &mut ctx.accounts.scope_b;
        let lg = &mut ctx.accounts.logbook;

        for i in 0..n {
            let z = ((a.focus ^ b.focus) as u64).wrapping_mul(1099511628211);
            a.focus = a.focus.checked_add(((z & 63) as u32) + 3).unwrap_or(u32::MAX);
            b.focus = b.focus.saturating_add((((z >> 6) & 63) as u32) + 5);
            lg.count = lg.count.saturating_add(1);
            lg.mix ^= z.rotate_left((i % 17) as u32);
        }

        let avg = if lg.count == 0 { 0 } else { (lg.mix / lg.count) as u32 };
        if avg > o.limit {
            o.mode = Sleep;
            a.slot ^= 1; b.slot = b.slot.saturating_add(1);
            lg.slot = lg.slot.saturating_add(1);
            msg!("sleep: slot tweaks & log slot++");
        } else {
            o.mode = Observe;
            a.focus = a.focus.saturating_add(9);
            b.focus = b.focus / 2 + 11;
            lg.mix ^= 0x0F0F_F0F0;
            msg!("observe: focus adjust & mix flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitObs<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub obs: Account<'info, Observatory>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub scope_a: Account<'info, Telescope>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub scope_b: Account<'info, Telescope>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 8)]
    pub logbook: Account<'info, Logbook>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub astronomer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Watch<'info> {
    #[account(mut, has_one=owner)]
    pub obs: Account<'info, Observatory>,
    #[account(
        mut,
        has_one=obs,
        constraint = scope_a.slot != scope_b.slot @ StarErr::Dup
    )]
    pub scope_a: Account<'info, Telescope>,
    #[account(
        mut,
        has_one=obs,
        constraint = scope_b.slot != logbook.slot @ StarErr::Dup
    )]
    pub scope_b: Account<'info, Telescope>,
    #[account(mut, has_one=obs)]
    pub logbook: Account<'info, Logbook>,
    pub astronomer: Signer<'info>,
}

#[account] pub struct Observatory { pub owner: Pubkey, pub limit: u32, pub mode: SkyMode }
#[account] pub struct Telescope   { pub obs: Pubkey, pub slot: u8, pub focus: u32 }
#[account] pub struct Logbook     { pub obs: Pubkey, pub slot: u8, pub count: u64, pub mix: u64 }

#[error_code] pub enum StarErr { #[msg("duplicate mutable account")] Dup }
