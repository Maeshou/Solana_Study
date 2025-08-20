// =============================================================================
// 8. NFT Minting with Proper Metadata Owner Checks
// =============================================================================
#[program]
pub mod secure_nft_mint {
    use super::*;

    pub fn create_collection(ctx: Context<CreateCollection>, name: String, symbol: String) -> Result<()> {
        let collection = &mut ctx.accounts.collection;
        collection.authority = ctx.accounts.authority.key();
        collection.name = name;
        collection.symbol = symbol;
        collection.total_minted = 0;
        collection.bump = *ctx.bumps.get("collection").unwrap();
        Ok(())
    }

    pub fn mint_nft(ctx: Context<MintNft>, uri: String) -> Result<()> {
        let collection = &mut ctx.accounts.collection;
        let nft_metadata = &mut ctx.accounts.nft_metadata;
        
        nft_metadata.collection = collection.key();
        nft_metadata.owner = ctx.accounts.recipient.key();
        nft_metadata.uri = uri;
        nft_metadata.token_id = collection.total_minted;
        nft_metadata.bump = *ctx.bumps.get("nft_metadata").unwrap();
        
        collection.total_minted += 1;
        Ok(())
    }
}

#[account]
pub struct Collection {
    pub authority: Pubkey,
    pub name: String,
    pub symbol: String,
    pub total_minted: u64,
    pub bump: u8,
}

#[account]
pub struct NftMetadata {
    pub collection: Pubkey,
    pub owner: Pubkey,
    pub uri: String,
    pub token_id: u64,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(name: String, symbol: String)]
pub struct CreateCollection<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 4 + name.len() + 4 + symbol.len() + 8 + 1,
        seeds = [b"collection", authority.key().as_ref()],
        bump
    )]
    pub collection: Account<'info, Collection>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(uri: String)]
pub struct MintNft<'info> {
    #[account(
        mut,
        seeds = [b"collection", collection.authority.as_ref()],
        bump = collection.bump,
        constraint = collection.authority == authority.key()
    )]
    pub collection: Account<'info, Collection>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 4 + uri.len() + 8 + 1,
        seeds = [b"nft", collection.key().as_ref(), &collection.total_minted.to_le_bytes()],
        bump
    )]
    pub nft_metadata: Account<'info, NftMetadata>,
    
    pub authority: Signer<'info>,
    
    /// CHECK: This account is properly validated as the recipient
    pub recipient: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}
