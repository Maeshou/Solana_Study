// ============================================================================
// 3) Rune Engraver — ルーン刻印（PDAなし）
//    防止: has_one / constraint 三連
// ============================================================================
declare_id!("RUNE34343434343434343434343434343434");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum EngravePhase { Draft, Carve, Polish }

#[program]
pub mod rune_engraver {
    use super::*;

    pub fn init_studio(ctx: Context<InitStudio>, threshold: u32) -> Result<()> {
        ctx.accounts.studio.artisan = ctx.accounts.artisan.key();
        ctx.accounts.studio.threshold = threshold;
        ctx.accounts.studio.phase = EngravePhase::Draft;
        Ok(())
    }

    pub fn engrave(ctx: Context<Engrave>, strokes: u32) -> Result<()> {
        for _ in 0..strokes {
            ctx.accounts.book.pages = ctx.accounts.book.pages.saturating_add(3);
            ctx.accounts.book.power = ctx.accounts.book.power.saturating_add(9);
            ctx.accounts.chisel.usage = ctx.accounts.chisel.usage.saturating_add(2);
        }

        if ctx.accounts.book.power > (ctx.accounts.studio.threshold as u64) {
            ctx.accounts.studio.phase = EngravePhase::Polish;
            ctx.accounts.chisel.sharpness = ctx.accounts.chisel.sharpness.saturating_add(5);
            ctx.accounts.book.pages = ctx.accounts.book.pages.saturating_add(2);
            msg!("power high: polish phase, improve chisel & pages");
        } else {
            ctx.accounts.studio.phase = EngravePhase::Carve;
            ctx.accounts.chisel.sharpness = ctx.accounts.chisel.sharpness.saturating_add(3);
            ctx.accounts.book.power = ctx.accounts.book.power.saturating_add(6);
            msg!("keep carving: boost sharpness & power");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStudio<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 4 + 1)]
    pub studio: Account<'info, Studio>,
    #[account(init, payer = payer, space = 8 + 4 + 8)]
    pub book: Account<'info, RuneBook>,
    #[account(init, payer = payer, space = 8 + 4 + 4)]
    pub chisel: Account<'info, Chisel>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub artisan: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Engrave<'info> {
    #[account(mut, has_one = artisan, constraint = studio.key() != book.key(), error = EngErr::Dup)]
    pub studio: Account<'info, Studio>,
    #[account(mut, constraint = book.key() != chisel.key(), error = EngErr::Dup)]
    pub book: Account<'info, RuneBook>,
    #[account(mut, constraint = studio.key() != chisel.key(), error = EngErr::Dup)]
    pub chisel: Account<'info, Chisel>,
    pub artisan: Signer<'info>,
}

#[account] pub struct Studio { pub artisan: Pubkey, pub threshold: u32, pub phase: EngravePhase }
#[account] pub struct RuneBook { pub pages: u32, pub power: u64 }
#[account] pub struct Chisel { pub sharpness: u32, pub usage: u32 }
#[error_code] pub enum EngErr { #[msg("dup")] Dup }