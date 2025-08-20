// ============================================================================
// 2) Beast Hatchery — 孵化管理（PDAなし / 子口座は has_one=hatchery）
// ============================================================================
declare_id!("BSTH22222222222222222222222222222222222222222");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum HatchMode { Warm, Active, Rest }

#[program]
pub mod beast_hatchery {
    use super::*;
    use HatchMode::*;

    pub fn init_hatchery(ctx: Context<InitHatchery>, temp: u32) -> Result<()> {
        let s = &mut ctx.accounts;
        s.hatchery.owner = s.owner.key();
        s.hatchery.target_temp = temp;
        s.hatchery.mode = Warm;

        s.egg_a.hatchery = s.hatchery.key();
        s.egg_b.hatchery = s.hatchery.key();
        s.journal.hatchery = s.hatchery.key();
        Ok(())
    }

    pub fn incubate(ctx: Context<Incubate>, days: u32) -> Result<()> {
        let s = &mut ctx.accounts;

        for d in 0..days {
            let inc = 5 + (d % 4);
            s.egg_a.energy = s.egg_a.energy.saturating_add(inc * 3);
            s.egg_b.energy = s.egg_b.energy.saturating_add(inc * 2 + 1);
            s.journal.stamps = s.journal.stamps.wrapping_add(1);
        }

        let heat = (s.egg_a.energy + s.egg_b.energy) / 2;
        if heat > s.hatchery.target_temp {
            s.hatchery.mode = Rest;
            s.journal.alerts = s.journal.alerts.wrapping_add(2);
            s.egg_a.energy = s.egg_a.energy / 2 + 9;
            s.egg_b.energy = s.egg_b.energy / 2 + 7;
            msg!("rest: alerts+2, energies damped");
        } else {
            s.hatchery.mode = Active;
            s.journal.notes = s.journal.notes.wrapping_add(3);
            s.egg_a.energy = s.egg_a.energy + 13;
            s.egg_b.energy = s.egg_b.energy + 11;
            msg!("active: notes+3, energy boosts");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitHatchery<'info> {
    #[account(init, payer=payer, space=8+32+4+1)]
    pub hatchery: Account<'info, Hatchery>,
    #[account(init, payer=payer, space=8+32+4)]
    pub egg_a: Account<'info, Egg>,
    #[account(init, payer=payer, space=8+32+4)]
    pub egg_b: Account<'info, Egg>,
    #[account(init, payer=payer, space=8+32+8+4)]
    pub journal: Account<'info, HatchLog>,
    #[account(mut)] pub payer: Signer<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Incubate<'info> {
    #[account(mut, has_one=owner)]
    pub hatchery: Account<'info, Hatchery>,
    #[account(
        mut,
        has_one=hatchery,
        constraint = egg_a.key() != egg_b.key() @ HErr::Dup
    )]
    pub egg_a: Account<'info, Egg>,
    #[account(mut, has_one=hatchery)]
    pub egg_b: Account<'info, Egg>,
    #[account(mut, has_one=hatchery, constraint = journal.key() != egg_a.key() @ HErr::Dup)]
    pub journal: Account<'info, HatchLog>,
    pub owner: Signer<'info>,
}

#[account] pub struct Hatchery { pub owner: Pubkey, pub target_temp: u32, pub mode: HatchMode }
#[account] pub struct Egg { pub hatchery: Pubkey, pub energy: u32 }
#[account] pub struct HatchLog { pub hatchery: Pubkey, pub stamps: u64, pub notes: u32, pub alerts: u32 }
#[error_code] pub enum HErr { #[msg("dup")] Dup }