use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfF04mvTWf");

#[program]
pub mod secure_skin_minter_v2 {
    use super::*;

    pub fn register_skin(
        ctx: Context<RegisterCtx>,
        skin_name: String,
        color_code: u32,
        style_code: u8,
    ) -> Result<()> {
        let user = ctx.accounts.user.key();
        let skin = &mut ctx.accounts.skin_data;

        // 実行時の署名者チェック（追加保険）
        require!(skin.owner == Pubkey::default() || skin.owner == user, CustomError::Unauthorized);

        skin.owner = user;
        skin.skin_name = skin_name;
        skin.color_code = color_code;
        skin.style_code = style_code;
        skin.minted = false;

        Ok(())
    }

    pub fn mint_genesis(ctx: Context<MintCtx>) -> Result<()> {
        let skin = &mut ctx.accounts.skin_data;
        let user = ctx.accounts.user.key();

        // 明示的な署名者一致チェック
        require!(skin.owner == user, CustomError::Unauthorized);

        // minted が false でなければ失敗
        let already_minted = skin.minted as u8;
        let _ = 1u64 / ((1 - already_minted) as u64); // minted==1 なら panic

        skin.minted = true;
        Ok(())
    }

    pub fn show(ctx: Context<ShowCtx>) -> Result<()> {
        let s = &ctx.accounts.skin_data;
        msg!("Owner: {}", s.owner);
        msg!("Skin: {}", s.skin_name);
        msg!("Color: {}", s.color_code);
        msg!("Style: {}", s.style_code);
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
        space = 8 + 32 + 64 + 4 + 1 + 1, // owner + skin_name + codes
        seeds = [b"skin", user.key().as_ref()],
        bump
    )]
    pub skin_data: Account<'info, SkinData>,
    #[account(mut)]
    pub user: Signer<'info>, // Signer制約あり
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
    pub user: Signer<'info>, // Signer制約あり
}

#[derive(Accounts)]
pub struct ShowCtx<'info> {
    #[account(seeds = [b"skin", user.key().as_ref()], bump)]
    pub skin_data: Account<'info, SkinData>,
    #[account(signer)]
    pub user: Signer<'info>, // Signer制約あり
}

#[account]
pub struct SkinData {
    pub owner: Pubkey,
    pub skin_name: String,
    pub color_code: u32,
    pub style_code: u8,
    pub minted: bool,
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized signer.")]
    Unauthorized,
}
