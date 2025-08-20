use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("RaidV3Vh8s4t1Vh8s4t1Vh8s4t1Vh8s4t1Vh8s4t1");

#[program]
pub mod raid_distribution_v3 {
    use super::*;

    pub fn init_boss(ctx: Context<InitBoss>, base_unit: u64, step: u16) -> Result<()> {
        let boss = &mut ctx.accounts.boss;
        boss.operator = ctx.accounts.operator.key();
        boss.base_reward = base_unit.max(2);
        boss.participant_step = step.min(2000).max(25);
        boss.kill_count = 3;                 // 0回避
        boss.max_combo_record = 7;           // 0回避
        boss.hard_mode_flag = false;
        Ok(())
    }

    pub fn act_distribute(ctx: Context<ActDistribute>, party_size: u8, combo_hits: u32) -> Result<()> {
        require!(party_size > 0, ErrRaid::NoParty);
        let boss = &mut ctx.accounts.boss;

        // 参加人数でボーナス増加（2人目以降は逓減）
        let mut bonus = 0u64;
        let mut i = 1u8;
        while i <= party_size {
            let add = (boss.participant_step as u64).saturating_div(i as u64);
            bonus = bonus.saturating_add(add);
            i = i.saturating_add(1);
        }

        // 難易度フラグを段階的に切替
        if combo_hits > boss.max_combo_record as u32 {
            boss.max_combo_record = combo_hits as u64;
        }
        if combo_hits >= 60 { boss.hard_mode_flag = true; }
        if combo_hits < 40 { boss.hard_mode_flag = false; }

        let mut payout = boss.base_reward.saturating_add(bonus);
        if boss.hard_mode_flag { payout = payout.saturating_mul(2); }

        token::transfer(ctx.accounts.treasury_to_player(), payout)?;
        boss.kill_count = boss.kill_count.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBoss<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 2 + 8 + 8 + 1)]
    pub boss: Account<'info, BossState>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActDistribute<'info> {
    #[account(mut, has_one = operator)]
    pub boss: Account<'info, BossState>,
    pub operator: Signer<'info>,
    #[account(mut)]
    pub raid_treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub player_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ActDistribute<'info> {
    pub fn treasury_to_player(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let accs = Transfer {
            from: self.raid_treasury.to_account_info(),
            to: self.player_wallet.to_account_info(),
            authority: self.operator.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accs)
    }
}

#[account]
pub struct BossState {
    pub operator: Pubkey,
    pub base_reward: u64,
    pub participant_step: u16,
    pub kill_count: u64,
    pub max_combo_record: u64,
    pub hard_mode_flag: bool,
}

#[error_code]
pub enum ErrRaid {
    #[msg("Party is empty")]
    NoParty,
}
