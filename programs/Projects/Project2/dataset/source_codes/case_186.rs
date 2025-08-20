use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct VoteTracker(pub u8, pub Vec<(u64, u64, u64)>); // (bump, Vec<(proposal_id, yes_count, no_count)>)

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzV7");

#[error_code]
pub enum ErrorCode {
    #[msg("Maximum number of proposals reached")]
    MaxProposalsReached,
    #[msg("Proposal not found")]
    ProposalNotFound,
}

#[program]
pub mod vote_tracker {
    use super::*;

    const MAX_PROPOSALS: usize = 30;

    /// アカウント初期化：内部 Vec は空のまま、bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let bump = *ctx.bumps.get("tracker").unwrap();
        ctx.accounts.tracker.0 = bump;
        Ok(())
    }

    /// 新規提案作成：件数制限チェック＋初期カウントで追加
    pub fn create_proposal(ctx: Context<Modify>, proposal_id: u64) -> Result<()> {
        let list = &mut ctx.accounts.tracker.1;
        if list.len() >= MAX_PROPOSALS {
            return err!(ErrorCode::MaxProposalsReached);
        }
        list.push((proposal_id, 0, 0));
        Ok(())
    }

    /// 投票：yes または no をカウント
    pub fn cast_vote(ctx: Context<Modify>, proposal_id: u64, yes: bool) -> Result<()> {
        let list = &mut ctx.accounts.tracker.1;
        let mut found = false;
        for entry in list.iter_mut() {
            if entry.0 == proposal_id {
                if yes {
                    entry.1 = entry.1.wrapping_add(1);
                } else {
                    entry.2 = entry.2.wrapping_add(1);
                }
                found = true;
            }
        }
        if !found {
            return err!(ErrorCode::ProposalNotFound);
        }
        Ok(())
    }

    /// 投票無しの提案を一括削除
    pub fn purge_empty(ctx: Context<Modify>) -> Result<()> {
        let list = &mut ctx.accounts.tracker.1;
        list.retain(|&(_, yes, no)| {
            if yes > 0 {
                true
            } else {
                if no > 0 {
                    true
                } else {
                    false
                }
            }
        });
        Ok(())
    }

    /// 賛成多数の提案数をログ出力
    pub fn count_passed(ctx: Context<Modify>) -> Result<()> {
        let list = &ctx.accounts.tracker.1;
        let mut cnt = 0u64;
        for &(_, yes, no) in list.iter() {
            if yes > no {
                cnt = cnt.wrapping_add(1);
            }
        }
        msg!("Passed proposals: {}", cnt);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"tracker", authority.key().as_ref()],
        bump,
        // discriminator(8) + bump(1) + Vec<len prefix 4 + max30*(8+8+8)>
        space = 8 + 1 + 4 + 30 * (8 + 8 + 8)
    )]
    pub tracker:   Account<'info, VoteTracker>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"tracker", authority.key().as_ref()],
        bump = tracker.0
    )]
    pub tracker:   Account<'info, VoteTracker>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
