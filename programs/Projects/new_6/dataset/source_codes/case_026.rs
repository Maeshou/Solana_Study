// 02. 進行度チェックと監査（監査官と対象ユーザの混同を防げていない構造）

use anchor_lang::prelude::*;

declare_id!("Pr0gr3ssAuditVuln0000000000000000000000000");

#[program]
pub mod vulnerable_progress_audit {
    use super::*;

    pub fn init_track(ctx: Context<InitTrack>, threshold: u32) -> Result<()> {
        let tracker = &mut ctx.accounts.tracker;
        tracker.owner = ctx.accounts.owner.key();
        tracker.threshold = threshold;
        tracker.total_events = 0;
        Ok(())
    }

    pub fn register_user(ctx: Context<RegisterUser>, lane: u8) -> Result<()> {
        let user = &mut ctx.accounts.user_card;
        user.tracker = ctx.accounts.tracker.key();
        user.progress = 0;
        user.lane = lane;
        Ok(())
    }

    pub fn audit_event(ctx: Context<AuditEvent>, deltas: Vec<i32>) -> Result<()> {
        let user = &mut ctx.accounts.user_card;
        let auditor = &mut ctx.accounts.auditor_card;
        let tracker = &mut ctx.accounts.tracker;
        let audit_log = &mut ctx.accounts.log;

        // 脆弱性: user_card と auditor_card に同一アカウントを渡しても弾かれない。
        // lane の一致／不一致、所有者、has_one、owner いずれも制約なし。

        let mut sum = 0i32;
        for d in deltas {
            sum = sum.saturating_add(d);
        }

        if sum > 0 {
            user.progress = user.progress.saturating_add(sum as u32);
            tracker.total_events += 1;
            audit_log.note = 0xAB;
        } else {
            auditor.progress = auditor.progress.saturating_sub((-sum) as u32);
            tracker.total_events += 2;
            audit_log.note = 0xCD;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTrack<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 4)]
    pub tracker: Account<'info, Tracker>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterUser<'info> {
    #[account(init, payer = registrar, space = 8 + 32 + 1 + 4)]
    pub user_card: Account<'info, ProgressCard>,
    #[account(mut)]
    pub registrar: Signer<'info>,
    pub tracker: Account<'info, Tracker>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AuditEvent<'info> {
    #[account(mut)]
    pub tracker: Account<'info, Tracker>,
    #[account(mut)]
    pub user_card: Account<'info, ProgressCard>,
    #[account(mut)]
    pub auditor_card: Account<'info, ProgressCard>,
    #[account(init_if_needed, payer = auditor_signer, space = 8 + 1)]
    pub log: Account<'info, AuditLog>,
    #[account(mut)]
    pub auditor_signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Tracker {
    pub owner: Pubkey,
    pub threshold: u32,
    pub total_events: u32,
}

#[account]
pub struct ProgressCard {
    pub tracker: Pubkey,
    pub lane: u8,
    pub progress: u32,
}

#[account]
pub struct AuditLog {
    pub note: u8,
}
