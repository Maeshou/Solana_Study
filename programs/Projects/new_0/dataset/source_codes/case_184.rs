use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct SavingsTracker(pub u8, pub Vec<(u64, u64)>); // (bump, Vec<(goal_id, saved_amount)>)

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzV5");

#[error_code]
pub enum ErrorCode {
    #[msg("Maximum number of goals reached")]
    MaxGoalsReached,
    #[msg("No such goal")]
    NoSuchGoal,
    #[msg("Insufficient saved amount")]
    InsufficientFunds,
}

#[program]
pub mod savings_tracker {
    use super::*;

    const MAX_GOALS: usize = 20;

    /// アカウント初期化：内部 Vec は空、bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let bump = *ctx.bumps.get("tracker").unwrap();
        ctx.accounts.tracker.0 = bump;
        Ok(())
    }

    /// 目標登録：件数制限チェック＋push
    pub fn add_goal(ctx: Context<Modify>, goal_id: u64) -> Result<()> {
        let list = &mut ctx.accounts.tracker.1;
        if list.len() >= MAX_GOALS {
            return err!(ErrorCode::MaxGoalsReached);
        }
        list.push((goal_id, 0));
        Ok(())
    }

    /// 貯金：既存目標を検索し、金額を加算
    pub fn deposit(ctx: Context<Modify>, goal_id: u64, amount: u64) -> Result<()> {
        let list = &mut ctx.accounts.tracker.1;
        let mut found = false;
        for entry in list.iter_mut() {
            if entry.0 == goal_id {
                entry.1 = entry.1.wrapping_add(amount);
                found = true;
            }
        }
        if !found {
            return err!(ErrorCode::NoSuchGoal);
        }
        Ok(())
    }

    /// 引き出し：既存目標を検索し、残高チェック＋減算
    pub fn withdraw(ctx: Context<Modify>, goal_id: u64, amount: u64) -> Result<()> {
        let list = &mut ctx.accounts.tracker.1;
        let mut found = false;
        for entry in list.iter_mut() {
            if entry.0 == goal_id {
                found = true;
                if entry.1 < amount {
                    return err!(ErrorCode::InsufficientFunds);
                }
                entry.1 = entry.1 - amount;
            }
        }
        if !found {
            return err!(ErrorCode::NoSuchGoal);
        }
        Ok(())
    }

    /// 完了・未達問わず残高 0 の目標を除去
    pub fn purge_zero(ctx: Context<Modify>) -> Result<()> {
        let list = &mut ctx.accounts.tracker.1;
        list.retain(|&(_, saved)| saved > 0);
        Ok(())
    }

    /// 目標数報告：ログ出力
    pub fn count_goals(ctx: Context<Modify>) -> Result<()> {
        let total = ctx.accounts.tracker.1.len() as u64;
        msg!("Total goals: {}", total);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"savings", authority.key().as_ref()],
        bump,
        // discriminator(8) + bump(1) + Vec<(u64,u64)> (max20: 4 + 20*(8+8))
        space = 8 + 1 + 4 + 20 * (8 + 8)
    )]
    pub tracker: Account<'info, SavingsTracker>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"savings", authority.key().as_ref()],
        bump = tracker.0
    )]
    pub tracker:   Account<'info, SavingsTracker>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
