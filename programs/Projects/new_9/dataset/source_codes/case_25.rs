use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("NFTGameVuln4444444444444444444444444444444");

#[program]
pub mod alchemy_lab {
    use super::*;

    // ポーションを醸造し、釜をクローズする
    pub fn brew_potion(ctx: Context<BrewPotion>) -> Result<()> {
        msg!("Potion brewing complete. The pot is now clean.");
        Ok(())
    }

    // 新しい錬金術の釜を準備する（脆弱な再生成処理）
    pub fn prepare_new_pot(ctx: Context<PrepareNewPot>, capacity: u32, purity: u8) -> Result<()> {
        let pot_account = ctx.accounts.alchemy_pot.to_account_info();
        let alchemist = ctx.accounts.alchemist.to_account_info();
        let pot_size: u64 = 64;

        let setup_fee = 800_000 + (capacity as u64 * 100) + (purity as u64 * 1000);

        anchor_lang::solana_program::program::invoke(
            &system_instruction::transfer(&alchemist.key(), &pot_account.key(), setup_fee),
            &[alchemist.clone(), pot_account.clone()],
        )?;
        anchor_lang::solana_program::program::invoke(
            &system_instruction::allocate(&pot_account.key(), pot_size),
            &[pot_account.clone()],
        )?;
        anchor_lang::solana_program::program::invoke(
            &system_instruction::assign(&pot_account.key(), &crate::id()),
            &[pot_account.clone()],
        )?;

        let mut data = pot_account.try_borrow_mut_data()?;
        let mut cursor = 8;
        data[cursor..cursor+4].copy_from_slice(&capacity.to_le_bytes());
        cursor += 4;
        data[cursor] = purity;
        cursor += 1;
        data[cursor] = 0; // Ingredients count
        Ok(())
    }
}

#[account]
pub struct AlchemyPot {
    pub liquid_capacity: u32,
    pub purity_level: u8,
    pub ingredients_count: u8,
}

#[derive(Accounts)]
pub struct PrepareNewPot<'info> {
    #[account(mut)]
    pub alchemy_pot: UncheckedAccount<'info>,
    #[account(mut)]
    pub alchemist: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BrewPotion<'info> {
    #[account(mut, close = alchemist)]
    pub alchemy_pot: Account<'info, AlchemyPot>,
    #[account(mut)]
    pub alchemist: Signer<'info>,
}