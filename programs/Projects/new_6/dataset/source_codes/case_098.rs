use anchor_lang::prelude::*;

declare_id!("NftCrAfTSt4ti0n3333333333333333333333333");

#[program]
pub mod nft_craft_station {
    use super::*;

    pub fn initialize_station(ctx: Context<InitializeStation>, max_energy: u16) -> Result<()> {
        let station = &mut ctx.accounts.station;
        station.owner = ctx.accounts.creator.key();
        station.energy = max_energy;
        station.cooldown = 0;
        Ok(())
    }

    pub fn craft_nft(ctx: Context<CraftNFT>, power_cost: u16, seed: u64) -> Result<()> {
        let station = &mut ctx.accounts.station;
        if station.energy < power_cost {
            station.cooldown += 1;
            return Ok(()); // no energy
        }

        station.energy -= power_cost;
        station.cooldown = 0;

        let mut crafted = NFTResult::default();
        if seed % 2 == 0 {
            crafted.rarity = "Epic".to_string();
            crafted.bonus = (seed % 50) as u8;
        } else {
            crafted.rarity = "Rare".to_string();
            crafted.bonus = (seed % 20) as u8;
        }

        let log = &mut ctx.accounts.log;
        log.last_result = crafted;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeStation<'info> {
    #[account(init, payer = creator, space = 64)]
    pub station: Account<'info, CraftStation>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub asset_account: AccountInfo<'info>, // Type Cosplay
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CraftNFT<'info> {
    #[account(mut)]
    pub station: Account<'info, CraftStation>,
    #[account(mut)]
    pub log: Account<'info, CraftLog>,
    pub asset_account: AccountInfo<'info>, // Type Cosplay - ambiguous
}

#[account]
pub struct CraftStation {
    pub owner: Pubkey,
    pub energy: u16,
    pub cooldown: u8,
}

#[account]
pub struct CraftLog {
    pub last_result: NFTResult,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone)]
pub struct NFTResult {
    pub rarity: String,
    pub bonus: u8,
}
