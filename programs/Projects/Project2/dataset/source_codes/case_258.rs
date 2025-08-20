use anchor_lang::prelude::*;

declare_id!("PetHun0777777777777777777777777777777777");

#[program]
pub mod pet_hunger {
    use super::*;

    pub fn feed(ctx: Context<Feed>, food: u8) -> Result<()> {
        let p = &mut ctx.accounts.pet;
        p.hunger = p.hunger.saturating_sub(food);
        if p.hunger == 0 {
            p.full_meals = p.full_meals.saturating_add(1);
        }
        Ok(())
    }

    pub fn starve(ctx: Context<Feed>, amount: u8) -> Result<()> {
        let p = &mut ctx.accounts.pet;
        p.hunger = (p.hunger + amount).min(p.max_hunger);
        p.starve_events = p.starve_events.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Feed<'info> {
    #[account(mut)]
    pub pet: Account<'info, PetData>,
}

#[account]
pub struct PetData {
    pub hunger: u8,
    pub max_hunger: u8,
    pub full_meals: u64,
    pub starve_events: u64,
}
