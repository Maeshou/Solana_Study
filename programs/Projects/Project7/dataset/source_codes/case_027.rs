use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("RaidV4f2Xk9Pq1Lm7Yd3AaBbCcDdEeFfGgHhIiJj002");

#[program]
pub mod raid_rewards_v4 {
    use super::*;

    pub fn init_boss(ctx: Context<InitBoss>, base_reward: u64, increment_step: u16) -> Result<()> {
        let boss_state = &mut ctx.accounts.boss_state;
        boss_state.operator_key = ctx.accounts.operator.key();
        boss_state.base_reward = base_reward.max(2);
        boss_state.increment_step = increment_step.min(1500).max(25);
        boss_state.clear_count = 2;
        boss_state.max_combo_record = 5;
        boss_state.challenge_flag = false;
        Ok(())
    }

    pub fn act_distribute(ctx: Context<ActDistribute>, party_count: u8, combo_points: u32) -> Result<()> {
        require!(party_count > 0, ErrRaid::NoParticipants);
        let boss_state = &mut ctx.accounts.boss_state;

        // 参加人数に応じて逓減加算
        let mut total_increment = 0u64;
        let mut member_index = 1u8;
        while member_index <= party_count {
            let add = (boss_state.increment_step as u64).saturating_div(member_index as u64);
            total_increment = total_increment.saturating_add(add);
            member_index = member_index.saturating_add(1);
        }

        // コンボによるフラグ更新
        if combo_points > boss_state.max_combo_record as u32 {
            boss_state.max_combo_record = combo_points as u64;
        }
        if combo_points >= 50 { boss_state.challenge_flag = true; }
        if combo_points < 30 { boss_state.challenge_flag = false; }

        let mut payout = boss_state.base_reward.saturating_add(total_increment);
        if boss_state.challenge_flag { payout = payout.saturating_mul(2); }

        token::transfer(ctx.accounts.treasury_to_player(), payout)?;
        boss_state.clear_count = boss_state.clear_count.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBoss<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 2 + 8 + 8 + 1)]
    pub boss_state: Account<'info, BossState>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActDistribute<'info> {
    #[account(mut, has_one = operator_key)]
    pub boss_state: Account<'info, BossState>,
    pub operator_key: Signer<'info>,

    #[account(mut)]
    pub reward_treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub player_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ActDistribute<'info> {
    pub fn treasury_to_player(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.reward_treasury.to_account_info(),
            to: self.player_vault.to_account_info(),
            authority: self.operator_key.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
}

#[account]
pub struct BossState {
    pub operator_key: Pubkey,
    pub base_reward: u64,
    pub increment_step: u16,
    pub clear_count: u64,
    pub max_combo_record: u64,
    pub challenge_flag: bool,
}

#[error_code]
pub enum ErrRaid {
    #[msg("No participants")]
    NoParticipants,
}
