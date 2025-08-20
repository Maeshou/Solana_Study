// 05. NFTカラー編集：アーティストと色管理者のなりすまし
use anchor_lang::prelude::*;

declare_id!("NFTc0l0r5555555555555555555555555555555555");

#[program]
pub mod nft_color_editor {
    use super::*;

    pub fn init_palette(ctx: Context<InitPalette>, base: u32, alpha: u8) -> Result<()> {
        let p = &mut ctx.accounts.palette;
        p.creator = ctx.accounts.artist.key();
        p.color_base = base;
        p.alpha = alpha;
        p.versions = 1;
        p.saturated = false;
        Ok(())
    }

    pub fn act_mutate_color(ctx: Context<MutateColor>, bump: u32) -> Result<()> {
        let p = &mut ctx.accounts.palette;
        let color_mgr = &ctx.accounts.manager;

        for i in 0..5 {
            p.color_base = p.color_base.wrapping_add(bump.wrapping_mul(i));
        }

        if p.alpha < 200 {
            p.alpha += 20;
        } else {
            p.alpha -= 30;
        }

        if p.versions > 10 {
            p.saturated = true;
        } else {
            p.versions += 1;
        }

        msg!("Color modified by: {}", color_mgr.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPalette<'info> {
    #[account(init, payer = artist, space = 8 + 32 + 4 + 1 + 4 + 1)]
    pub palette: Account<'info, Palette>,
    #[account(mut)]
    pub artist: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MutateColor<'info> {
    #[account(mut)]
    pub palette: Account<'info, Palette>,
    /// CHECK: Type Cosplay: managerの型チェックなし
    pub manager: AccountInfo<'info>,
}

#[account]
pub struct Palette {
    pub creator: Pubkey,
    pub color_base: u32,
    pub alpha: u8,
    pub versions: u32,
    pub saturated: bool,
}
