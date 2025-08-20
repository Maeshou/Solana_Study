// 04. NFTクラフト用素材消費プログラム
use anchor_lang::prelude::*;
use anchor_spl::token::*;

declare_id!("Craft444444444444444444444444444444444444444");

#[program]
pub mod nft_crafting {
    use super::*;

    pub fn init_crafting_station(
        ctx: Context<InitCraftingStation>,
        base_material_cost: u64,
        success_rate: u8,
    ) -> Result<()> {
        let station = &mut ctx.accounts.crafting_station;
        station.crafter = ctx.accounts.crafter.key();
        station.material_mint = ctx.accounts.material_mint.key();
        station.base_material_cost = base_material_cost;
        station.success_rate = success_rate;
        station.total_crafts_attempted = 0;
        station.successful_crafts = 0;
        station.materials_consumed = 0;
        station.station_type = StationType::Basic;
        Ok(())
    }

    pub fn execute_craft_attempt(
        ctx: Context<ExecuteCraftAttempt>,
        recipe_type: CraftingRecipe,
        enhancement_level: u8,
    ) -> Result<()> {
        let station = &mut ctx.accounts.crafting_station;
        let random_seed = Clock::get()?.unix_timestamp as u64;
        
        // レシピに応じた素材消費量計算
        let recipe_multiplier = match recipe_type {
            CraftingRecipe::Common => 1,
            CraftingRecipe::Rare => 3,
            CraftingRecipe::Epic => 7,
            CraftingRecipe::Legendary => 15,
        };

        let enhancement_multiplier = (enhancement_level as u64).saturating_add(1);
        let total_material_cost = station.base_material_cost * recipe_multiplier * enhancement_multiplier;

        // 成功率計算とタイプアップグレードチェック
        let mut success_chance = station.success_rate;
        if station.successful_crafts > 10 && station.station_type == StationType::Basic {
            station.station_type = StationType::Advanced;
            success_chance = success_chance.saturating_add(20);
        }

        // 素材消費ループ（分割バーン）
        let mut remaining_cost = total_material_cost;
        while remaining_cost > 0 {
            let burn_amount = std::cmp::min(remaining_cost, 50_000 * 10u64.pow(6));
            
            burn(
                ctx.accounts.burn_materials_ctx(),
                burn_amount,
            )?;
            
            remaining_cost -= burn_amount;
        }

        station.materials_consumed += total_material_cost;
        station.total_crafts_attempted += 1;

        // 成功判定
        let success = (random_seed % 100) < success_chance as u64;
        if success {
            station.successful_crafts += 1;
            
            // NFT mint処理（成功時のみ）
            mint_to(
                ctx.accounts.mint_nft_ctx(),
                1,
            )?;
        }

        Ok(())
    }
}

impl<'info> ExecuteCraftAttempt<'info> {
    fn burn_materials_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Burn {
                mint: self.material_mint.to_account_info(),
                from: self.crafter_material_account.to_account_info(),
                authority: self.crafter.to_account_info(),
            }
        )
    }

    fn mint_nft_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.nft_mint.to_account_info(),
                to: self.crafter_nft_account.to_account_info(),
                authority: self.crafting_station.to_account_info(),
            }
        )
    }
}

#[derive(Accounts)]
pub struct InitCraftingStation<'info> {
    #[account(mut)]
    pub crafter: Signer<'info>,
    
    #[account(
        init,
        payer = crafter,
        space = 8 + CraftingStation::INIT_SPACE,
        seeds = [b"station", crafter.key().as_ref()],
        bump
    )]
    pub crafting_station: Account<'info, CraftingStation>,
    
    pub material_mint: Account<'info, Mint>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ExecuteCraftAttempt<'info> {
    #[account(mut)]
    pub crafter: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"station", crafting_station.crafter.as_ref()],
        bump
    )]
    pub crafting_station: Account<'info, CraftingStation>,
    
    #[account(mut)]
    pub material_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub nft_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub crafter_material_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub crafter_nft_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(InitSpace)]
pub struct CraftingStation {
    pub crafter: Pubkey,
    pub material_mint: Pubkey,
    pub base_material_cost: u64,
    pub success_rate: u8,
    pub total_crafts_attempted: u32,
    pub successful_crafts: u32,
    pub materials_consumed: u64,
    pub station_type: StationType,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum StationType {
    Basic,
    Advanced,
    Master,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace)]
pub enum CraftingRecipe {
    Common,
    Rare,
    Epic,
    Legendary,
}