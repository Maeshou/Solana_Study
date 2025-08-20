// Example 1: NFT Character Retirement and Revival
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_instruction, program::invoke_signed};

declare_id!("GameChar1111111111111111111111111111111");

#[program]
pub mod nft_character_revival {
    use super::*;

    pub fn retire_character(ctx: Context<RetireCharacter>) -> Result<()> {
        let character_data = &ctx.accounts.character_pda;
        msg!("Retiring character with level: {}", character_data.level);
        Ok(())
    }

    pub fn revive_character_with_seed(
        ctx: Context<ReviveCharacter>,
        player_seed: [u8; 16],
        stored_bump: u8,
        new_stats: CharacterStats,
    ) -> Result<()> {
        let character_info = ctx.accounts.character_pda.to_account_info();
        
        let transfer_ix = system_instruction::transfer(
            &ctx.accounts.funding_wallet.key(),
            &character_info.key(),
            3_500_000
        );
        anchor_lang::solana_program::program::invoke(
            &transfer_ix,
            &[ctx.accounts.funding_wallet.to_account_info(), character_info.clone()],
        )?;

        let seed_array: &[&[u8]] = &[b"character", &player_seed, &[stored_bump]];
        
        let allocate_ix = system_instruction::allocate(&character_info.key(), 1024);
        invoke_signed(&allocate_ix, &[character_info.clone()], &[seed_array])?;
        
        let assign_ix = system_instruction::assign(&character_info.key(), &crate::id());
        invoke_signed(&assign_ix, &[character_info.clone()], &[seed_array])?;

        let mut account_data = character_info.try_borrow_mut_data()?;
        let character_bytes = bytemuck::bytes_of(&new_stats);
        for index in 0..character_bytes.len() {
            account_data[index] = character_bytes[index];
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RetireCharacter<'info> {
    #[account(mut, seeds = [b"character", owner.key().as_ref()], bump, close = refund_target)]
    pub character_pda: Account<'info, GameCharacter>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub refund_target: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct ReviveCharacter<'info> {
    #[account(mut)]
    pub character_pda: UncheckedAccount<'info>,
    #[account(mut)]
    pub funding_wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GameCharacter {
    pub level: u32,
    pub experience: u64,
    pub health_points: u32,
}

#[derive(Clone, Copy)]
pub struct CharacterStats {
    pub level: u32,
    pub experience: u64,
    pub health_points: u32,
}

unsafe impl bytemuck::Pod for CharacterStats {}
unsafe impl bytemuck::Zeroable for CharacterStats {}
