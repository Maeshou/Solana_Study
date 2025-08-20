use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, Token, TokenAccount, Mint};

declare_id!("DailyV3x8kR2DailyV3x8kR2DailyV3x8kR2Da42");

#[program]
pub mod daily_quest_v3 {
    use super::*;

    pub fn init_board(ctx: Context<InitBoard>, max_today: u64, week_base: u64) -> Result<()> {
        let b = &mut ctx.accounts.board;
        b.owner = ctx.accounts.owner.key();
        b.daily_max = max_today.max(9);
        b.issued_today = week_base.saturating_div(7).max(2);
        b.weekly_base = week_base.max(10);
        b.streak = 2;
        Ok(())
    }

    pub fn act_claim(ctx: Context<ActClaim>, tasks_done: u8, proof_cost: u64) -> Result<()> {
        let b = &mut ctx.accounts.board;

        // 証跡消費（ burn ）
        token::burn(ctx.accounts.burn_ctx(), proof_cost.max(1))?;

        // 1日の上限を超えない範囲で段階付与（タスク1つごとに+α）
        let mut reward = b.weekly_base.saturating_div(20).max(1);
        let mut i = 0u8;
        while i < tasks_done {
            reward = reward.saturating_add((i as u64 % 3).saturating_add(1));
            i = i.saturating_add(1);
        }

        let projected = b.issued_today.saturating_add(reward);
        if projected > b.daily_max {
            // 溢れた分は翌日に繰越（ここではstate更新のみ）
            let overflow = projected.saturating_sub(b.daily_max);
            b.weekly_base = b.weekly_base.saturating_add(overflow);
            b.streak = 1;
            return Err(ErrDaily::OverLimit.into());
        }

        token::mint_to(ctx.accounts.mint_ctx(), reward)?;
        b.issued_today = projected;
        b.streak = b.streak.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBoard<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub board: Account<'info, BoardState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActClaim<'info> {
    #[account(mut, has_one = owner)]
    pub board: Account<'info, BoardState>,
    pub owner: Signer<'info>,

    pub proof_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_proof: Account<'info, TokenAccount>,

    pub reward_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_reward: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ActClaim<'info> {
    pub fn burn_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let a = Burn {
            mint: self.proof_mint.to_account_info(),
            from: self.user_proof.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let a = MintTo {
            mint: self.reward_mint.to_account_info(),
            to: self.user_reward.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
}

#[account]
pub struct BoardState {
    pub owner: Pubkey,
    pub daily_max: u64,
    pub issued_today: u64,
    pub weekly_base: u64,
    pub streak: u64,
}

#[error_code]
pub enum ErrDaily {
    #[msg("Daily limit exceeded")]
    OverLimit,
}
