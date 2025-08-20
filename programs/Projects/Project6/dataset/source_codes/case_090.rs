// (5) Quest Tracker — 進行度/チャプター管理
use anchor_lang::prelude::*;
declare_id!("Qu3s7Tr4ck3r555555555555555555555555555555");

#[program]
pub mod quest_tracker {
    use super::*;
    use Badge::*;

    pub fn init_story(ctx: Context<InitStory>, title: String) -> Result<()> {
        let s = &mut ctx.accounts.story;
        s.owner = ctx.accounts.author.key();
        s.title = title;
        s.steps = 0;
        Ok(())
    }

    pub fn init_badge(ctx: Context<InitBadge>, badge: Badge) -> Result<()> {
        let b = &mut ctx.accounts.badge;
        b.story = ctx.accounts.story.key();
        b.kind = badge;
        b.points = 0;
        b.ring = [0; 5];
        Ok(())
    }

    pub fn progress(ctx: Context<Progress>, seed: u64) -> Result<()> {
        let s = &mut ctx.accounts.story;
        let a = &mut ctx.accounts.achiever;
        let c = &mut ctx.accounts.counter;
        let l = &mut ctx.accounts.log;

        let mut v = seed;
        for i in 0..5 {
            v = v.rotate_left(11) ^ 0xA5A5_5A5A_1122_3344;
            a.ring[i] = a.ring[i].saturating_add(((v >> (i * 6)) as u32) & 0x3F);
        }

        if a.kind == Star {
            a.points = a.points.saturating_add(((v & 0xFFFF) as u32) + 7);
            s.steps = s.steps.saturating_add(1);
            l.count = l.count.saturating_add(1);
            l.last = l.last ^ v;
            msg!("Star badge path updated");
        } else {
            c.points = c.points.saturating_add((((v >> 5) & 0x7FFF) as u32) + 3);
            s.steps = s.steps.saturating_add(1);
            l.count = l.count.saturating_add(2);
            l.last = l.last.wrapping_add(v);
            msg!("Non-star badge path updated");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStory<'info> {
    #[account(init, payer = author, space = 8 + Story::MAX)]
    pub story: Account<'info, Story>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct InitBadge<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub story: Account<'info, Story>,
    #[account(init, payer = user, space = 8 + BadgeAccount::MAX)]
    pub badge: Account<'info, BadgeAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Progress<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub story: Account<'info, Story>,
    #[account(mut, has_one = story, owner = crate::ID)]
    pub log: Account<'info, Log>,
    #[account(mut, has_one = story, owner = crate::ID)]
    pub achiever: Account<'info, BadgeAccount>,
    #[account(
        mut,
        has_one = story,
        owner = crate::ID,
        constraint = achiever.kind != counter.kind @ ErrCode::CosplayBlocked
    )]
    pub counter: Account<'info, BadgeAccount>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Story { pub owner: Pubkey, pub title: String, pub steps: u64 }
impl Story { pub const MAX: usize = 32 + 4 + 64 + 8; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Badge { Star, Moon, Sun }
use Badge::*;

#[account]
pub struct BadgeAccount { pub story: Pubkey, pub kind: Badge, pub points: u32, pub ring: [u32; 5] }
impl BadgeAccount { pub const MAX: usize = 32 + 1 + 4 + 4 * 5; }

#[account]
pub struct Log { pub story: Pubkey, pub last: u64, pub count: u32 }
impl Log { pub const MAX: usize = 32 + 8 + 4; }

#[error_code]
pub enum ErrCode { #[msg("Type Cosplay blocked by badge mismatch")] CosplayBlocked }
