// (4) Palette Studio — アート/色パレットの承認フロー
use anchor_lang::prelude::*;
declare_id!("Pa1e77eStuD104444444444444444444444444444");

#[program]
pub mod palette_studio {
    use super::*;
    use RoleTag::*;

    pub fn init_studio(ctx: Context<InitStudio>) -> Result<()> {
        let s = &mut ctx.accounts.studio;
        s.owner = ctx.accounts.admin.key();
        s.revision = 0;
        Ok(())
    }

    pub fn init_profile(ctx: Context<InitProfile>, role: RoleTag) -> Result<()> {
        let p = &mut ctx.accounts.profile;
        p.studio = ctx.accounts.studio.key();
        p.role = role;
        p.score = 0;
        p.hist = [0; 6];
        Ok(())
    }

    pub fn process_palette(ctx: Context<ProcessPalette>, hues: [u16; 6]) -> Result<()> {
        let st = &mut ctx.accounts.studio;
        let a = &mut ctx.accounts.artist;
        let r = &mut ctx.accounts.reviewer;
        let q = &mut ctx.accounts.queue;

        let mut sum = 0u64;
        for i in 0..6 {
            let v = (hues[i] as u64) * ((i as u64) + 1);
            sum = sum.saturating_add(v & 0x3FFF);
            a.hist[i] = a.hist[i].saturating_add((v as u32) & 0xFF);
        }

        if a.role == Creator {
            a.score = a.score.saturating_add((sum % 101) as u32);
            st.revision = st.revision.saturating_add(1);
            q.depth = q.depth.saturating_add(1);
            q.last = q.last.rotate_left(3) ^ sum;
            msg!("Creator path applied");
        } else {
            r.score = r.score.saturating_add(((sum >> 2) % 67) as u32);
            st.revision = st.revision.saturating_add(1);
            q.depth = q.depth.saturating_add(2);
            q.last = q.last.wrapping_add(sum.rotate_right(5));
            msg!("Reviewer path applied");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStudio<'info> {
    #[account(init, payer = admin, space = 8 + Studio::MAX)]
    pub studio: Account<'info, Studio>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct InitProfile<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub studio: Account<'info, Studio>,
    #[account(init, payer = user, space = 8 + Profile::MAX)]
    pub profile: Account<'info, Profile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ProcessPalette<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub studio: Account<'info, Studio>,
    #[account(mut, has_one = studio, owner = crate::ID)]
    pub queue: Account<'info, Queue>,
    #[account(mut, has_one = studio, owner = crate::ID)]
    pub artist: Account<'info, Profile>,
    #[account(
        mut,
        has_one = studio,
        owner = crate::ID,
        constraint = artist.role != reviewer.role @ ErrCode::CosplayBlocked
    )]
    pub reviewer: Account<'info, Profile>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Studio { pub owner: Pubkey, pub revision: u64 }
impl Studio { pub const MAX: usize = 32 + 8; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum RoleTag { Creator, Curator, Auditor }
use RoleTag::*;

#[account]
pub struct Profile { pub studio: Pubkey, pub role: RoleTag, pub score: u32, pub hist: [u32; 6] }
impl Profile { pub const MAX: usize = 32 + 1 + 4 + 4 * 6; }

#[account]
pub struct Queue { pub studio: Pubkey, pub last: u64, pub depth: u32 }
impl Queue { pub const MAX: usize = 32 + 8 + 4; }

#[error_code]
pub enum ErrCode { #[msg("Type Cosplay blocked by role mismatch")] CosplayBlocked }
