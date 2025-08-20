use anchor_lang::prelude::*;

declare_id!("PetEvo0888888888888888888888888888888888");

#[program]
pub mod pet_evolution {
    use super::*;

    pub fn evolve(ctx: Context<Evolve>, trigger: EvolutionTrigger) -> Result<()> {
        let p = &mut ctx.accounts.pet;
        p.stage = match trigger {
            EvolutionTrigger::LevelUp if p.level >= 10 => EvolutionStage::Adult,
            EvolutionTrigger::ItemUse if p.items_used >= 5 => EvolutionStage::Winged,
            _ => p.stage,
        };
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum EvolutionTrigger {
    LevelUp,
    ItemUse,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum EvolutionStage {
    Baby,
    Adult,
    Winged,
}

#[derive(Accounts)]
pub struct Evolve<'info> {
    #[account(mut)]
    pub pet: Account<'info, PetData>,
}

#[account]
pub struct PetData {
    pub level: u8,
    pub items_used: u8,
    pub stage: EvolutionStage,
}
