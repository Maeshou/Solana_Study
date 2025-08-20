// 6. Artist Community
declare_id!("A2R6T9I3S7T1C5O9M4M8U2N6I0T4Y8E1D5");

use anchor_lang::prelude::*;

#[program]
pub mod artist_community_insecure {
    use super::*;

    pub fn create_community(ctx: Context<CreateCommunity>, community_id: u64, max_artists: u32) -> Result<()> {
        let community = &mut ctx.accounts.community;
        community.founder = ctx.accounts.founder.key();
        community.community_id = community_id;
        community.artist_count = 0;
        community.max_artists = max_artists;
        community.is_moderated = false;
        msg!("Artist community {} created.", community.community_id);
        Ok(())
    }

    pub fn register_artist(ctx: Context<RegisterArtist>, artist_id: u32, initial_rating: u8) -> Result<()> {
        let artist = &mut ctx.accounts.artist;
        let community = &mut ctx.accounts.community;
        
        if community.artist_count < community.max_artists {
            artist.current_rating = initial_rating;
            artist.is_active = ArtistStatus::Active;
            community.artist_count = community.artist_count.saturating_add(1);
            msg!("Artist {} registered successfully.", artist.artist_id);
        } else {
            artist.is_active = ArtistStatus::Inactive;
            msg!("Community is full. Artist {} registration failed.", artist.artist_id);
        }

        artist.community = community.key();
        artist.artist_id = artist_id;
        artist.wallet_address = ctx.accounts.artist_wallet.key();
        Ok(())
    }

    pub fn rate_artworks(ctx: Context<RateArtworks>, rating: u8) -> Result<()> {
        let artist1 = &mut ctx.accounts.artist1;
        let artist2 = &mut ctx.accounts.artist2;
        
        if matches!(artist1.is_active, ArtistStatus::Active) && matches!(artist2.is_active, ArtistStatus::Active) {
            artist1.current_rating = artist1.current_rating.saturating_add(rating).min(255);
            artist2.current_rating = artist2.current_rating.saturating_sub(rating).max(0);
            msg!("Artist 1 rating increased, artist 2 rating decreased.");
        } else {
            msg!("One or both artists are not active.");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCommunity<'info> {
    #[account(init, payer = founder, space = 8 + 32 + 8 + 4 + 4 + 1)]
    pub community: Account<'info, ArtistCommunity>,
    #[account(mut)]
    pub founder: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterArtist<'info> {
    #[account(mut, has_one = community)]
    pub community: Account<'info, ArtistCommunity>,
    #[account(init, payer = artist_wallet, space = 8 + 32 + 4 + 32 + 1 + 1)]
    pub artist: Account<'info, Artist>,
    #[account(mut)]
    pub artist_wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RateArtworks<'info> {
    #[account(mut, has_one = community)]
    pub community: Account<'info, ArtistCommunity>,
    #[account(mut, has_one = community)]
    pub artist1: Account<'info, Artist>,
    #[account(mut, has_one = community)]
    pub artist2: Account<'info, Artist>,
}

#[account]
pub struct ArtistCommunity {
    pub founder: Pubkey,
    pub community_id: u64,
    pub artist_count: u32,
    pub max_artists: u32,
    pub is_moderated: bool,
}

#[account]
pub struct Artist {
    pub community: Pubkey,
    pub artist_id: u32,
    pub wallet_address: Pubkey,
    pub current_rating: u8,
    pub is_active: ArtistStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum ArtistStatus {
    Active,
    Inactive,
}
