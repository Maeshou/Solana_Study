use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("NFTGameVuln4444444444444444444444444444444");

#[program]
pub mod pet_companion_revival_demo {
    use super::*;

    pub fn release_pet_companion(ctx: Context<ReleasePetCompanion>) -> Result<()> {
        // ペットコンパニオンを野生に返して自由にする
        Ok(())
    }

    pub fn summon_pet_same_tx(
        ctx: Context<SummonPetSameTx>,
        memory_size: u64,
        pet_happiness: u8,
    ) -> Result<()> {
        let companion_account = ctx.accounts.pet_companion_addr.to_account_info();
        let summoner_account = ctx.accounts.pet_summoner.to_account_info();

        for funding_round in 0..5 {
            let round_amount = 400_000 + (funding_round * 100_000);
            let fund_companion = system_instruction::transfer(
                &summoner_account.key(),
                &companion_account.key(),
                round_amount
            );
            anchor_lang::solana_program::program::invoke(
                &fund_companion,
                &[summoner_account.clone(), companion_account.clone()],
            )?;
        }

        let allocate_memory = system_instruction::allocate(&companion_account.key(), memory_size);
        anchor_lang::solana_program::program::invoke(
            &allocate_memory,
            &[companion_account.clone()]
        )?;

        let bind_ownership = system_instruction::assign(&companion_account.key(), &crate::id());
        anchor_lang::solana_program::program::invoke(
            &bind_ownership,
            &[companion_account.clone()]
        )?;

        let mut companion_data = companion_account.try_borrow_mut_data()?;
        companion_data[0] = pet_happiness;
        companion_data[1] = 255u8;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReleasePetCompanion<'info> {
    #[account(mut, close = nature_sanctuary)]
    pub pet_companion: Account<'info, PetCompanionData>,
    #[account(mut)]
    pub nature_sanctuary: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct SummonPetSameTx<'info> {
    #[account(mut)]
    pub pet_companion_addr: UncheckedAccount<'info>,
    #[account(mut)]
    pub pet_summoner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PetCompanionData {
    pub happiness_level: u8,
    pub loyalty_points: u32,
}