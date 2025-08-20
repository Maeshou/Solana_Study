use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqDailyCh01");

#[program]
pub mod nft_daily_challenge {
    use super::*;

    pub fn claim_daily_challenge(
        ctx: Context<ClaimDailyChallenge>,
        reward_amount: u64,
    ) -> Result<()> {
        let prog_acc = &mut ctx.accounts.challenge_progress_account.to_account_info();
        let data     = &mut prog_acc.data.borrow_mut();

        if data.len() < 16 {
            return err!(ErrorCode::DataTooShort);
        }

        let (ts_slice, streak_slice) = data.split_at_mut(8);
        let now = Clock::get()?.unix_timestamp as u64;
        let last = u64::from_le_bytes(ts_slice.try_into().unwrap());

        // 24時間経過していなければエラー、経過していれば streak を 1 にリセット
        if now < last.saturating_add(86400) {
            return err!(ErrorCode::TooSoon);
        }

        let new_streak = 1;

        // タイムスタンプと streak を更新
        ts_slice.copy_from_slice(&now.to_le_bytes());
        streak_slice.copy_from_slice(&new_streak.to_le_bytes());

        // 報酬支払い処理
        **ctx.accounts.reward_pool.to_account_info().lamports.borrow_mut() =
            ctx.accounts.reward_pool.to_account_info().lamports()
            .saturating_sub(reward_amount);
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() += reward_amount;

        msg!(
            "Daily challenge claimed: streak={} reward={} by {}",
            new_streak,
            reward_amount,
            ctx.accounts.user.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimDailyChallenge<'info> {
    #[account(mut)]
    pub challenge_progress_account: AccountInfo<'info>,
    #[account(mut)]
    pub reward_pool: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが想定より短いです")]
    DataTooShort,
    #[msg("24時間まだ経過していません")]
    TooSoon,
}
