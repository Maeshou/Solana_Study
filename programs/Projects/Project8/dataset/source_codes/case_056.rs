use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

// (パターン8のdeclare_id, StakeRecord を流用)
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_staking_game {
    use super::*;
    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        let stake_record = &mut ctx.accounts.stake_record;
        
        if !stake_record.is_active {
            return err!(StakingError::AlreadyClaimed);
        }

        let clock = Clock::get()?;
        let staking_duration = clock.unix_timestamp - stake_record.start_time;
        
        // 10秒ごとに1トークン獲得する単純な計算
        let reward_amount = staking_duration as u64 / 10;
        
        // 報酬がゼロ以上の場合のみ処理
        if reward_amount > 0 {
            msg!("Staked for {} seconds.", staking_duration);
            msg!("Claiming {} reward tokens.", reward_amount);
            // ここで実際にトークンをユーザーに転送する処理（invoke）が入る
        }
        
        // 報酬がゼロの場合
        if reward_amount == 0 {
            msg!("Not enough staking duration to claim rewards.");
        }
        
        stake_record.is_active = false;
        msg!("Stake record has been closed.");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(
        mut,
        seeds = [b"stake", owner.key().as_ref(), stake_record.nft_mint.as_ref()],
        bump = stake_record.bump,
        has_one = owner
    )]
    pub stake_record: Account<'info, StakeRecord>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct StakeRecord {
    pub owner: Pubkey,
    pub nft_mint: Pubkey,
    pub start_time: i64,
    pub is_active: bool,
    pub bump: u8,
}

#[error_code]
pub enum StakingError {
    #[msg("Rewards have already been claimed for this stake.")]
    AlreadyClaimed,
}