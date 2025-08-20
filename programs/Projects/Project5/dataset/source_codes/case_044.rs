// 3) Star Garden FP — Q24.8 で生長（PDAあり）
declare_id!("SGFP333333333333333333333333333333333");
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum GardenState { Seed, Bloom, Trim }
use GardenState::*;

#[program]
pub mod star_garden_fp {
    use super::*;
    pub fn init_garden(ctx: Context<InitGarden>, cap: u32) -> Result<()> {
        let g = &mut ctx.accounts;
        g.cfg.keeper = g.keeper.key();
        g.cfg.cap_q24 = cap << 8; // Q24.8
        g.cfg.state = Seed;
        Ok(())
    }
    pub fn grow(ctx: Context<Growing>, days: u32) -> Result<()> {
        let g = &mut ctx.accounts;
        for _ in 0..days {
            // height_q24 += height_q24/8 + 256 (1.0)
            let inc = (g.tree.height_q24 >> 3) + 256;
            g.tree.height_q24 = g.tree.height_q24.saturating_add(inc);
            // hue回転
            g.tree.hue = g.tree.hue.rotate_left(3) ^ (g.tree.hue >> 2);
            // 記録
            g.journal.stamps = g.journal.stamps.wrapping_add(1);
        }
        if g.tree.height_q24 > g.cfg.cap_q24 {
            g.cfg.state = Trim;
            g.tree.height_q24 = g.cfg.cap_q24;
            g.journal.prunes = g.journal.prunes.wrapping_add(2);
            g.tree.hue ^= 0x0F0F;
            msg!("trimmed: clip height, +prunes, hue xor");
        } else {
            g.cfg.state = Bloom;
            g.journal.notes = g.journal.notes.wrapping_add(3);
            g.tree.height_q24 = g.tree.height_q24 + 128; // +0.5
            g.tree.hue = g.tree.hue.wrapping_add(7);
            msg!("bloom: notes+3, +0.5 height, hue+7");
        }
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitGarden<'info> {
    #[account(init, payer=payer, space=8+32+4+1, seeds=[b"cfg", keeper.key().as_ref()], bump)]
    pub cfg: Account<'info, GardenCfg>,
    #[account(init, payer=payer, space=8+4+4)]
    pub tree: Account<'info, TreeQ24>,
    #[account(init, payer=payer, space=8+4+4)]
    pub journal: Account<'info, GardenJournal>,
    #[account(mut)] pub payer: Signer<'info>,
    pub keeper: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Growing<'info> {
    #[account(mut, seeds=[b"cfg", keeper.key().as_ref()], bump, has_one=keeper)]
    pub cfg: Account<'info, GardenCfg>,
    #[account(mut, constraint = cfg.key() != tree.key(), error = SgfpErr::Dup)]
    pub tree: Account<'info, TreeQ24>,
    #[account(mut)]
    pub journal: Account<'info, GardenJournal>,
    pub keeper: Signer<'info>,
}
#[account] pub struct GardenCfg { pub keeper: Pubkey, pub cap_q24: u32, pub state: GardenState }
#[account] pub struct TreeQ24 { pub height_q24: u32, pub hue: u32 }
#[account] pub struct GardenJournal { pub stamps: u32, pub prunes: u32, pub notes: u32 }
#[error_code] pub enum SgfpErr { #[msg("dup")] Dup }