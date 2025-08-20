// ============================================================================
// 1) Avatar Atelier — アバター染色（PDAあり・bump未保存）
//    防止: constraint / has_one / seeds + assert_ne!
// ============================================================================
declare_id!("AVAT12121212121212121212121212121212");
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum DyeMode { Mono, Gradient, Prism }

#[program]
pub mod avatar_atelier {
    use super::*;

    pub fn init_atelier(ctx: Context<InitAtelier>, limit: u32) -> Result<()> {
        ctx.accounts.atelier.artist = ctx.accounts.artist.key();
        ctx.accounts.atelier.limit = limit;
        ctx.accounts.atelier.mode = DyeMode::Gradient;

        // palette と avatar の数値系はゼロ初期化に任せる（0/1 代入の多発を回避）
        Ok(())
    }

    pub fn apply_dyes(ctx: Context<ApplyDyes>, passes: u8) -> Result<()> {
        // 追加の衝突防止（属性に加えてプログラム内でも）
        assert_ne!(ctx.accounts.atelier.key(), ctx.accounts.palette.key(), "atelier/palette must differ");

        for _ in 0..passes {
            ctx.accounts.avatar.layers = ctx.accounts.avatar.layers.saturating_add(2);
            ctx.accounts.avatar.hue = ctx.accounts.avatar.hue.saturating_add(3);
            ctx.accounts.palette.dyes = ctx.accounts.palette.dyes.saturating_add(5);
        }

        if ctx.accounts.avatar.hue > ctx.accounts.atelier.limit {
            ctx.accounts.atelier.mode = DyeMode::Mono;
            ctx.accounts.palette.rarity = ctx.accounts.palette.rarity.saturating_add(4);
            ctx.accounts.palette.temperature = ctx.accounts.palette.temperature.saturating_add(7);
            msg!("limit exceeded: switch to Mono, rarity/temperature boosted");
        } else {
            ctx.accounts.atelier.mode = DyeMode::Prism;
            ctx.accounts.avatar.layers = ctx.accounts.avatar.layers.saturating_add(1);
            ctx.accounts.palette.temperature = ctx.accounts.palette.temperature.saturating_add(2);
            msg!("within limit: switch to Prism, fine-tune layers & temperature");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAtelier<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 4 + 1)]
    pub atelier: Account<'info, Atelier>,
    #[account(init, payer = payer, space = 8 + 32 + 4 + 1)]
    pub avatar: Account<'info, Avatar>,
    #[account(init, payer = payer, space = 8 + 8 + 1 + 4, seeds = [b"palette", artist.key().as_ref()], bump)]
    pub palette: Account<'info, Palette>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub artist: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApplyDyes<'info> {
    #[account(mut, has_one = artist)]
    pub atelier: Account<'info, Atelier>,
    #[account(mut, constraint = avatar.key() != palette.key(), error = DyeErr::Dup)]
    pub avatar: Account<'info, Avatar>,
    #[account(mut, seeds = [b"palette", artist.key().as_ref()], bump)]
    pub palette: Account<'info, Palette>,
    pub artist: Signer<'info>,
}

#[account] pub struct Atelier { pub artist: Pubkey, pub limit: u32, pub mode: DyeMode }
#[account] pub struct Avatar { pub owner: Pubkey, pub hue: u32, pub layers: u8 }
#[account] pub struct Palette { pub dyes: u64, pub rarity: u8, pub temperature: u32 }
#[error_code] pub enum DyeErr { #[msg("duplicate mutable account")] Dup }
