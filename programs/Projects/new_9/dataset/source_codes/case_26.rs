use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("NFTGameVuln5555555555555555555555555555555");

#[program]
pub mod farmville_revival {
    use super::*;

    // 新しい畑を耕す（脆弱な再生成処理）
    pub fn plow_new_field(ctx: Context<PlowNewField>, fertility: u16, water_level: u32) -> Result<()> {
        let plot_account = ctx.accounts.farmland_plot.to_account_info();
        let farmer = ctx.accounts.farmer.to_account_info();
        let plot_size = 128;

        let land_tax = 400_000 + (fertility as u64 * 500);
        anchor_lang::solana_program::program::invoke(
            &system_instruction::transfer(&farmer.key(), &plot_account.key(), land_tax),
            &[farmer.clone(), plot_account.clone()],
        )?;

        let instructions = [
            system_instruction::allocate(&plot_account.key(), plot_size),
            system_instruction::assign(&plot_account.key(), &crate::id()),
        ];
        anchor_lang::solana_program::program::invoke(&instructions[0], &[plot_account.clone()])?;
        anchor_lang::solana_program::program::invoke(&instructions[1], &[plot_account.clone()])?;

        let mut data = plot_account.try_borrow_mut_data()?;
        data[8..10].copy_from_slice(&fertility.to_le_bytes());
        data[10..14].copy_from_slice(&water_level.to_le_bytes());
        data[14] = 0; // growth_stage
        Ok(())
    }

    // 作物を収穫し、土地をクローズする
    pub fn harvest_crops(ctx: Context<HarvestCrops>) -> Result<()> {
        msg!("Crops harvested. The plot is now fallow.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct HarvestCrops<'info> {
    #[account(mut, close = farmer)]
    pub farmland_plot: Account<'info, FarmlandPlot>,
    #[account(mut)]
    pub farmer: Signer<'info>,
}

#[account]
pub struct FarmlandPlot {
    pub fertility: u16,
    pub water_level: u32,
    pub growth_stage: u8,
}

#[derive(Accounts)]
pub struct PlowNewField<'info> {
    #[account(mut)]
    pub farmland_plot: UncheckedAccount<'info>,
    #[account(mut)]
    pub farmer: Signer<'info>,
    pub system_program: Program<'info, System>,
}