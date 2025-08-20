
use anchor_lang::prelude::*;

declare_id!("AssetMGR11111111111111111111111111111111111");

#[program]
pub mod case5 {
    use super::*;

    pub fn set_asset_description(ctx: Context<SetAssetDescription>, desc: String) -> Result<()> {
        let asset = &mut ctx.accounts.asset;
        asset.description = desc;
        asset.revision += 1;
        msg!("Asset description updated: {}", asset.description);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetAssetDescription<'info> {
    #[account(mut)]
    pub asset: Account<'info, Asset>,
    /// CHECK: missing signer and matching check
    pub editor: UncheckedAccount<'info>,
}

#[account]
pub struct Asset {
    pub owner: Pubkey,
    pub description: String,
    pub revision: u64,
}
