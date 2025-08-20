use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfF05mvTWf");

#[program]
pub mod skin_with_image_004 {
    use super::*;

    pub fn register_skin_with_image(
        ctx: Context<RegisterCtx>,
        skin_name: String,
        color_code: u32,
        style_code: u8,
        image_uri: String,
    ) -> Result<()> {
        let user = ctx.accounts.user.key();
        let skin = &mut ctx.accounts.skin_data;

        skin.owner = user;
        skin.skin_name = skin_name;
        skin.color_code = color_code;
        skin.style_code = style_code;
        skin.image_uri = image_uri;
        skin.minted = false;

        Ok(())
    }

    pub fn mint_genesis(ctx: Context<MintCtx>) -> Result<()> {
        let skin = &mut ctx.accounts.skin_data;
        let user = ctx.accounts.user.key();

        require!(skin.owner == user, CustomError::Unauthorized);

        // 再ミント防止
        let already_minted = skin.minted as u8;
        let _ = 1u64 / ((1 - already_minted) as u64); // すでにtrueならpanic

        skin.minted = true;
        Ok(())
    }

    pub fn show(ctx: Context<ShowCtx>) -> Result<()> {
        let s = &ctx.accounts.skin_data;
        msg!("Skin Owner: {}", s.owner);
        msg!("Skin Name: {}", s.skin_name);
        msg!("Color: {}", s.color_code);
        msg!("Style: {}", s.style_code);
        msg!("Image URI: {}", s.image_uri);
        msg!("Minted: {}", s.minted);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct RegisterCtx<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 64 + 4 + 1 + 1 + 128, // skin name + image uri
        seeds = [b"skin", user.key().as_ref()],
        bump
    )]
    pub skin_data: Account<'info, SkinData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintCtx<'info> {
    #[account(
        mut,
        seeds = [b"skin", user.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub skin_data: Account<'info, SkinData>,
    #[account(signer)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ShowCtx<'info> {
    #[account(seeds = [b"skin", user.key().as_ref()], bump)]
    pub skin_data: Account<'info, SkinData>,
    #[account(signer)]
    pub user: Signer<'info>,
}

#[account]
pub struct SkinData {
    pub owner: Pubkey,
    pub skin_name: String,
    pub color_code: u32,
    pub style_code: u8,
    pub image_uri: String,  // IPFS hash or external image URI
    pub minted: bool,
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized signer.")]
    Unauthorized,
}
