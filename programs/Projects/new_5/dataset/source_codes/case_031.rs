// ============================================================================
// 7) Content Review Queue (two mutable drafts)
// ============================================================================
use anchor_lang::prelude::*;

declare_id!("REV77777777777777777777777777777777777777");

#[program]
pub mod content_review {
    use super::*;
    use Verdict::*;

    pub fn init_moderator(ctx: Context<InitModerator>, level: u8) -> Result<()> {
        let m = &mut ctx.accounts.moderator;
        m.owner = ctx.accounts.owner.key();
        m.level = level;
        m.queue = 0;
        m.bias = 50;
        Ok(())
    }

    pub fn init_draft(ctx: Context<InitDraft>, tag: u32) -> Result<()> {
        let d = &mut ctx.accounts.draft;
        d.parent = ctx.accounts.moderator.key();
        d.tag = tag;
        d.score = 0;
        d.verdict = Pending;
        d.flags = 0;
        Ok(())
    }

    pub fn judge_pair(ctx: Context<JudgePair>, weight: u32) -> Result<()> {
        let m = &mut ctx.accounts.moderator;
        let d1 = &mut ctx.accounts.first;
        let d2 = &mut ctx.accounts.second;

        // bias adjustment loop
        for t in 0..5 {
            let jitter = ((m.level as u32 + weight + t) % 9) as u32;
            m.bias = (m.bias + jitter).min(100);
            m.queue = m.queue.saturating_add(1);
        }

        if d1.tag & 2 == 0 {
            d1.verdict = Approved;
            d1.score = d1.score.saturating_add(weight / 2 + (m.level as u32));
            d1.flags = d1.flags.saturating_sub(d1.flags.min(1));
            m.queue = m.queue.saturating_sub(1);
            msg!("D1 approved; score={}, queue={}", d1.score, m.queue);
        } else {
            d1.verdict = Rejected;
            d1.score = d1.score / 2 + 3;
            d1.flags = d1.flags.saturating_add(2);
            m.bias = m.bias.saturating_sub(1);
            msg!("D1 rejected; score={}, bias={}", d1.score, m.bias);
        }

        for _ in 0..3 {
            if d2.score & 1 == 1 {
                d2.verdict = Approved;
                d2.score = d2.score.saturating_add((m.bias % 13) as u32 + 1);
                m.queue = m.queue.saturating_sub(1);
                msg!("D2 approve+; score={}, queue={}", d2.score, m.queue);
            } else {
                d2.verdict = Pending;
                d2.flags = d2.flags.saturating_add(1);
                m.bias = m.bias / 2 + 10;
                msg!("D2 pending; flags={}, bias={}", d2.flags, m.bias);
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitModerator<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 1 + 8 + 4)]
    pub moderator: Account<'info, Moderator>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitDraft<'info> {
    #[account(mut)]
    pub moderator: Account<'info, Moderator>,
    #[account(init, payer = author, space = 8 + 32 + 4 + 4 + 1 + 4)]
    pub draft: Account<'info, Draft>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JudgePair<'info> {
    #[account(mut)]
    pub moderator: Account<'info, Moderator>,
    #[account(mut, has_one = parent)]
    pub first: Account<'info, Draft>,
    #[account(mut, has_one = parent)]
    pub second: Account<'info, Draft>, // can alias
}

#[account]
pub struct Moderator {
    pub owner: Pubkey,
    pub level: u8,
    pub queue: u64,
    pub bias: u32,
}

#[account]
pub struct Draft {
    pub parent: Pubkey,
    pub tag: u32,
    pub score: u32,
    pub verdict: Verdict,
    pub flags: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Verdict {
    Pending,
    Approved,
    Rejected,
}
use Verdict::*;

#[error_code]
pub enum ReviewError {
    #[msg("moderation error")]
    ModerationError,
}
