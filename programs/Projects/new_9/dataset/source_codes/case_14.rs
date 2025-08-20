use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("NFTGameVuln2222222222222222222222222222222");

#[program]
pub mod nft_weapon_revival_demo {
    use super::*;

    pub fn destroy_weapon_nft(ctx: Context<DestroyWeaponNft>) -> Result<()> {
        // 武器NFTを破壊して素材を回収
        Ok(())
    }

    pub fn recreate_weapon_same_tx(
        ctx: Context<RecreateWeaponSameTx>,
        allocation_size: u64,
        weapon_damage: u16,
    ) -> Result<()> {
        let weapon_account = ctx.accounts.weapon_nft_addr.to_account_info();
        let funding_account = ctx.accounts.material_collector.to_account_info();

        while weapon_account.lamports() < 3_000_000 {
            let transfer_amount = 500_000;
            let fund_weapon = system_instruction::transfer(
                &funding_account.key(),
                &weapon_account.key(),
                transfer_amount
            );
            anchor_lang::solana_program::program::invoke(
                &fund_weapon,
                &[funding_account.clone(), weapon_account.clone()],
            )?;
        }

        let resize_account = system_instruction::allocate(&weapon_account.key(), allocation_size);
        anchor_lang::solana_program::program::invoke(
            &resize_account,
            &[weapon_account.clone()]
        )?;

        let transfer_ownership = system_instruction::assign(&weapon_account.key(), &crate::id());
        anchor_lang::solana_program::program::invoke(
            &transfer_ownership,
            &[weapon_account.clone()]
        )?;

        let mut weapon_data = weapon_account.try_borrow_mut_data()?;
        weapon_data[0] = (weapon_damage & 0xff) as u8;
        weapon_data[1] = ((weapon_damage >> 8) & 0xff) as u8;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DestroyWeaponNft<'info> {
    #[account(mut, close = material_vault)]
    pub weapon_nft: Account<'info, WeaponNftData>,
    #[account(mut)]
    pub material_vault: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct RecreateWeaponSameTx<'info> {
    #[account(mut)]
    pub weapon_nft_addr: UncheckedAccount<'info>,
    #[account(mut)]
    pub material_collector: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct WeaponNftData {
    pub damage: u16,
    pub durability: u8,
    pub element_type: u8,
}