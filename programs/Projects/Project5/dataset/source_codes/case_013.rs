// ============================================================================
// 4) LandKeeper（土地管理）— PDA不使用 + constraint三連 + require!
// ============================================================================
declare_id!("LAND44444444444444444444444444444444");

#[program]
pub mod land_keeper {
    use super::*;

    pub fn init_plot(ctx: Context<InitPlot>, fert: u32) -> Result<()> {
        ctx.accounts.plot.owner = ctx.accounts.farmer.key();
        ctx.accounts.plot.fertility = fert;
        ctx.accounts.plot.planted = false;

        ctx.accounts.profile.rank = 1;
        ctx.accounts.profile.energy = 100;
        ctx.accounts.profile.boots = true;

        ctx.accounts.harvest.days = 0;
        ctx.accounts.harvest.yield_est = 0;
        Ok(())
    }

    pub fn cultivate(ctx: Context<Cultivate>, days: u32) -> Result<()> {
        require!(ctx.accounts.plot.key() != ctx.accounts.profile.key(), LandErr::Dup);
        require!(ctx.accounts.plot.key() != ctx.accounts.harvest.key(), LandErr::Dup);
        require!(ctx.accounts.profile.key() != ctx.accounts.harvest.key(), LandErr::Dup);

        for _ in 0..days {
            ctx.accounts.profile.energy = ctx.accounts.profile.energy.saturating_sub(1);
            ctx.accounts.harvest.days = ctx.accounts.harvest.days.saturating_add(1);
            ctx.accounts.harvest.yield_est = ctx.accounts.harvest.yield_est.saturating_add(ctx.accounts.plot.fertility as u64);
        }

        if ctx.accounts.profile.energy < 50 {
            ctx.accounts.plot.planted = false;
            ctx.accounts.profile.boots = false;
            ctx.accounts.harvest.yield_est = ctx.accounts.harvest.yield_est.saturating_add(5);
            msg!("tired: pause");
        } else {
            ctx.accounts.plot.planted = true;
            ctx.accounts.profile.boots = true;
            ctx.accounts.harvest.yield_est = ctx.accounts.harvest.yield_est.saturating_add(1);
            msg!("steady work");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPlot<'info> {
    #[account(init, payer=payer, space=8+32+4+1)]
    pub plot: Account<'info, Plot>,
    #[account(init, payer=payer, space=8+4+4+1)]
    pub profile: Account<'info, Profile>,
    #[account(init, payer=payer, space=8+4+8)]
    pub harvest: Account<'info, Harvest>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub farmer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Cultivate<'info> {
    #[account(mut, constraint = plot.key() != profile.key(), error = LandErr::Dup)]
    pub plot: Account<'info, Plot>,
    #[account(mut)]
    pub profile: Account<'info, Profile>,
    #[account(mut)]
    pub harvest: Account<'info, Harvest>,
}

#[account] pub struct Plot { pub owner: Pubkey, pub fertility: u32, pub planted: bool }
#[account] pub struct Profile { pub rank: u32, pub energy: u32, pub boots: bool }
#[account] pub struct Harvest { pub days: u32, pub yield_est: u64 }

#[error_code] pub enum LandErr { #[msg("dup")] Dup }

