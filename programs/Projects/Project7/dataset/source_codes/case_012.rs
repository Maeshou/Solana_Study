use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("RaidPayV200000000000000000000000000000000");

#[program]
pub mod raid_reward_payout_v2 {
    use super::*;

    pub fn init_boss(ctx: Context<InitBoss>, base_reward: u64, combo_step: u16) -> Result<()> {
        let boss_state = &mut ctx.accounts.boss_state;
        boss_state.operator = ctx.accounts.operator.key();
        boss_state.default_reward = base_reward;
        boss_state.combo_step = combo_step.min(1000);
        boss_state.defeats = 1;
        boss_state.max_combo_seen = 1;
        boss_state.hard_flag = false;
        Ok(())
    }

    pub fn act_distribute(ctx: Context<ActDistribute>, combo: u32, party_size: u8) -> Result<()> {
        require!(party_size > 0, RaidErr::EmptyParty);

        let boss_state = &mut ctx.accounts.boss_state;

        // ループ：パーティ人数で加算ボーナス
        let mut group_bonus = 0u64;
        for _ in 0..party_size {
            group_bonus = group_bonus.saturating_add(boss_state.combo_step as u64);
        }

        // 分岐：最大コンボ更新と難易度フラグ
        if combo > boss_state.max_combo_seen {
            boss_state.max_combo_seen = combo as u64;
        }
        if combo >= 40 {
            boss_state.hard_flag = true;
        } else {
            boss_state.hard_flag = false;
        }

        let mut payout = boss_state.default_reward.saturating_add(group_bonus);
        if boss_state.hard_flag {
            payout = payout.saturating_mul(2);
        }

        let cpi = ctx.accounts.transfer_reward();
        token::transfer(cpi, payout)?;

        boss_state.defeats = boss_state.defeats.saturating_add(1);
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
    #[account(mut, has_one = operator)]
    pub boss_state: Account<'info, BossState>,
    pub operator: Signer<'info>,

    #[account(mut)]
    pub reward_treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub player_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ActDistribute<'info> {
    pub fn transfer_reward(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let accs = Transfer {
            from: self.reward_treasury.to_account_info(),
            to: self.player_vault.to_account_info(),
            authority: self.operator.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accs)
    }
}

#[account]
pub struct BossState {
    pub operator: Pubkey,
    pub default_reward: u64,
    pub combo_step: u16,
    pub defeats: u64,
    pub max_combo_seen: u64,
    pub hard_flag: bool,
}

#[error_code]
pub enum RaidErr {
    #[msg("No party members")]
    EmptyParty,
}
