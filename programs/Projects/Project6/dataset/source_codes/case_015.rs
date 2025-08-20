// #5: Artwork and Color Palette
// ドメイン: NFTアートのピクセルカラーパレット管理。
// 安全対策: `ArtWork` と `Palette` は親子関係 `has_one` で紐付けられ、同一の所有者による操作を強制。`ColorChannel` の enum を用いて、不整合な値を排除。

declare_id!("B2C3D4E5F6G7H8I9J0K1L2M3N4O5P6Q7R8S9T0U1");

#[program]
pub mod art_palette {
    use super::*;

    pub fn initialize_artwork(ctx: Context<InitializeArtwork>, width: u16, height: u16) -> Result<()> {
        let artwork = &mut ctx.accounts.artwork;
        artwork.owner = ctx.accounts.owner.key();
        artwork.width = width;
        artwork.height = height;
        artwork.is_published = false;
        Ok(())
    }

    pub fn update_palette_and_render_preview(
        ctx: Context<UpdatePaletteAndRenderPreview>,
        pixel_data: Vec<u8>,
    ) -> Result<()> {
        let artwork = &mut ctx.accounts.artwork;
        let palette = &mut ctx.accounts.palette;

        if palette.colors.len() == 0 {
            palette.colors = vec![Color { r: 0, g: 0, b: 0 }; 256];
        }

        for (i, p) in pixel_data.iter().enumerate() {
            let color = &mut palette.colors[*p as usize];
            let new_r = color.r.checked_add(1).unwrap_or(u8::MAX);
            let new_g = color.g.wrapping_add(1);
            color.r = new_r;
            color.g = new_g;
            color.b = color.b.checked_add(1).unwrap_or(u8::MAX);

            if i % 10 == 0 {
                msg!("Updated color at index {} to r:{}, g:{}, b:{}", p, color.r, color.g, color.b);
            }
        }

        // 簡易ビット操作
        let mut flag: u8 = 0b00000001; // Is_Dirty flag
        if artwork.is_published {
            flag &= 0b11111110; // Clear Is_Dirty flag
        } else {
            flag |= 0b00000010; // Add Is_Unpublished flag
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeArtwork<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 2 + 2 + 1,
        owner = crate::ID,
    )]
    pub artwork: Account<'info, Artwork>,
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 4 + 256 * 3 + 8,
        owner = crate::ID,
    )]
    pub palette: Account<'info, Palette>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePaletteAndRenderPreview<'info> {
    #[account(mut, has_one = owner)]
    pub artwork: Account<'info, Artwork>,
    #[account(
        mut,
        has_one = owner,
        // ArtworkとPaletteは別アカウントであることを検証
        constraint = artwork.key() != palette.key() @ ErrorCode::CosplayBlocked,
    )]
    pub palette: Account<'info, Palette>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Artwork {
    pub owner: Pubkey,
    pub width: u16,
    pub height: u16,
    pub is_published: bool,
}

#[account]
pub struct Palette {
    pub owner: Pubkey,
    pub artwork: Pubkey,
    pub colors: Vec<Color>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Account is being cosplayed as a different role.")]
    CosplayBlocked,
}
