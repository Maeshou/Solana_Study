use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_game {
    use super::*;

    // 土地から資源を収穫する
    pub fn harvest_from_land(ctx: Context<HarvestFromLand>) -> Result<()> {
        let land = &mut ctx.accounts.land_plot;
        let player_resources = &mut ctx.accounts.player_resources;
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;

        // 最後に収穫してからの経過時間
        let time_elapsed = current_timestamp.saturating_sub(land.last_harvest_timestamp);
        
        // 収穫クールダウン（例: 3600秒）が経過しているか
        require!(time_elapsed > 3600, GameError::HarvestOnCooldown);

        // 収穫量を計算（土地のレアリティと経過時間に基づく）
        let mut harvest_amount: u64 = 0;
        for _ in 0..(time_elapsed / 60) { // 1分ごとに計算
            if land.rarity == 1 { // Common
                 harvest_amount += 10;
            }
            if land.rarity == 2 { // Rare
                 harvest_amount += 25;
            }
            if land.rarity == 3 { // Legendary
                 harvest_amount += 70;
            }
        }
        
        // プレイヤーの資源アカウントに収穫物を追加
        let resource_id_to_add = land.resource_type;
        let mut resource_updated = false;
        for resource in player_resources.resources.iter_mut() {
            if resource.resource_id == resource_id_to_add {
                resource.amount += harvest_amount;
                resource_updated = true;
            }
        }

        if !resource_updated {
             player_resources.resources.push(Resource {
                resource_id: resource_id_to_add,
                amount: harvest_amount,
            });
        }
        
        // 最終収穫時間を更新
        land.last_harvest_timestamp = current_timestamp;

        msg!("Harvested {} of resource {}!", harvest_amount, resource_id_to_add);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct HarvestFromLand<'info> {
    #[account(mut, seeds = [b"land", owner.key().as_ref(), land_plot.mint.as_ref()], bump = land_plot.bump)]
    pub land_plot: Account<'info, LandPlot>,
    #[account(mut, seeds = [b"resources", owner.key().as_ref()], bump = player_resources.bump)]
    pub player_resources: Account<'info, PlayerResources>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct LandPlot {
    pub mint: Pubkey,
    pub rarity: u8, // 1: Common, 2: Rare, 3: Legendary
    pub resource_type: u32,
    pub last_harvest_timestamp: i64,
    pub bump: u8,
}

#[account]
pub struct PlayerResources {
    pub resources: Vec<Resource>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Resource {
    pub resource_id: u32,
    pub amount: u64,
}

#[error_code]
pub enum GameError {
    #[msg("Harvest is still on cooldown.")]
    HarvestOnCooldown,
}