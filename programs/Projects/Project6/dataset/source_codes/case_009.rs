// ===============================================
// (9) content_review: コンテンツ審査（投稿・モデレ・ログ）
//   - 多層防御: has_one + lane 不一致
// ===============================================
use anchor_lang::prelude::*;
declare_id!("C0nTenTreV999999999999999999999999999999");

#[program]
pub mod content_review {
    use super::*;

    pub fn init_post(ctx: Context<InitPost>, lane: u8) -> Result<()> {
        let p = &mut ctx.accounts.post;
        p.owner = ctx.accounts.owner.key();
        p.lane = lane;
        p.score = 0;
        Ok(())
    }

    pub fn init_moderator(ctx: Context<InitModerator>, lane: u8) -> Result<()> {
        let m = &mut ctx.accounts.moderator;
        m.parent = ctx.accounts.post.key();
        m.lane = lane;
        m.vote = 0;
        Ok(())
    }

    pub fn init_log(ctx: Context<InitLog>) -> Result<()> {
        let l = &mut ctx.accounts.log;
        l.ok = 0;
        l.ng = 0;
        l.events = [0u32; 4];
        Ok(())
    }

    pub fn review(ctx: Context<Review>, weight: u16) -> Result<()> {
        let p = &mut ctx.accounts.post;
        let a = &mut ctx.accounts.mod_a;
        let b = &mut ctx.accounts.mod_b;
        let l = &mut ctx.accounts.log;

        // 2人の投票を取り込み（lane ミスマッチが前提）
        let delta_a = ((weight as u32) ^ (a.lane as u32)) & 0x3FF;
        let delta_b = ((weight as u32) ^ (b.lane as u32).rotate_left(3)) & 0x3FF;

        p.score = p.score.saturating_add(delta_a / 3).saturating_add(delta_b / 4);

        // イベントログのローリング更新
        for i in 0..l.events.len() {
            let v = l.events[i].wrapping_add(((p.score as u32) >> i) & 0x7F);
            l.events[i] = v;
        }

        if (p.score & 1) == 0 {
            l.ok = l.ok.saturating_add(1);
        } else {
            l.ng = l.ng.saturating_add(1);
        }
        Ok(())
    }
}

// -------------------- Accounts --------------------

#[derive(Accounts)]
pub struct InitPost<'info> {
    #[account(
        init,
        payer = owner,
        // 8 + 32(owner) + 1(lane) + 4(score)
        space = 8 + 32 + 1 + 4
    )]
    pub post: Account<'info, Post>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitModerator<'info> {
    #[account(mut)]
    pub post: Account<'info, Post>,
    #[account(
        init,
        payer = owner,
        // 8 + 32(parent) + 1(lane) + 4(vote)
        space = 8 + 32 + 1 + 4
    )]
    pub moderator: Account<'info, Moderator>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitLog<'info> {
    #[account(
        init,
        payer = owner,
        // 8 + 4(ok) + 4(ng) + (4*4)(events)
        space = 8 + 4 + 4 + (4*4)
    )]
    pub log: Account<'info, Log>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Review<'info> {
    #[account(mut)]
    pub post: Account<'info, Post>,
    #[account(
        mut,
        constraint = mod_a.parent == post.key() @ ReviewErr::Cosplay
    )]
    pub mod_a: Account<'info, Moderator>,
    #[account(
        mut,
        constraint = mod_b.parent == post.key() @ ReviewErr::Cosplay,
        constraint = mod_a.lane != mod_b.lane @ ReviewErr::Cosplay
    )]
    pub mod_b: Account<'info, Moderator>,
    #[account(mut)]
    pub log: Account<'info, Log>,
}

// -------------------- Data --------------------

#[account]
pub struct Post {
    pub owner: Pubkey,
    pub lane: u8,
    pub score: u32,
}

#[account]
pub struct Moderator {
    pub parent: Pubkey, // = post
    pub lane: u8,
    pub vote: u32,
}

#[account]
pub struct Log {
    pub ok: u32,
    pub ng: u32,
    pub events: [u32; 4],
}

#[error_code]
pub enum ReviewErr { #[msg("cosplay blocked")] Cosplay }
