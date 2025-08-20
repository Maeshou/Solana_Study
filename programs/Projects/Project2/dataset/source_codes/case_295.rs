use anchor_lang::prelude::*;

declare_id!("EnergyDrink0044444444444444444444444444444444");

#[program]
pub mod energy_drink {
    use super::*;

    pub fn drink(ctx: Context<Drink>) -> Result<()> {
        let stats = &mut ctx.accounts.stats;
        stats.drinks_used = stats.drinks_used.saturating_add(1);
        if stats.drinks_used % stats.bonus_interval == 0 {
            stats.bonus_energy = stats.bonus_energy.saturating_add(stats.bonus_amount);
        }
        Ok(())
    }

    pub fn reset(ctx: Context<Drink>) -> Result<()> {
        let stats = &mut ctx.accounts.stats;
        stats.drinks_used = 0;
        stats.bonus_energy = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Drink<'info> {
    #[account(mut)]
    pub stats: Account<'info, DrinkStats>,
}

#[account]
pub struct DrinkStats {
    pub drinks_used: u64,
    pub bonus_interval: u64,
    pub bonus_amount: u64,
    pub bonus_energy: u64,
}
