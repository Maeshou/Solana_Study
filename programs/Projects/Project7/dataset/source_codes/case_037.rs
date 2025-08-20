use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("RaidB02Nf7Qw4Ls8Pd2Kj9Um1Ax6Ye3Vr5Tc0B002");

#[program]
pub mod raid_rewards_segment_v1 {
    use super::*;

    pub fn init_boss(ctx: Context<InitBoss>, base_reward: u64) -> Result<()> {
        let boss_state = &mut ctx.accounts.boss_state;
        boss_state.operator = ctx.accounts.operator.key();
        boss_state.base_reward = if base_reward < 2 { 2 } else { base_reward };
        boss_state.clear_count = 2;
        boss_state.max_combo_seen = 5;
        boss_state.challenge_flag = false;
        Ok(())
    }

    pub fn act_distribute(ctx: Context<ActDistribute>, party_size: u8, combo_value: u32) -> Result<()> {
        require!(party_size > 0, RaidErr::NoMembers);

        let boss_state = &mut ctx.accounts.boss_state;
        let mut group_bonus: u64 = 0;
        let mut party_index: u8 = 1;

        // 区間別の加算（=if 式は使わず、順次 if）
        while party_index <= party_size {
            if party_index <= 3 { group_bonus = group_bonus + 8; }
            if party_index > 3 { group_bonus = group_bonus + 5; }
            if party_index > 7 { group_bonus = group_bonus + 2; }
            party_index = party_index + 1;
        }

        // 逐次平方根近似（ニュートン法風）
        let mut sqrt_estimate: u64 = 1;
        let mut iteration_step: u8 = 0;
        while iteration_step < 6 {
            let divisor = sqrt_estimate;
            let adjusted = if divisor < 1 { 1 } else { divisor };
            sqrt_estimate = (sqrt_estimate + ((combo_value as u64) / adjusted)) / 2;
            iteration_step = iteration_step + 1;
        }

        if combo_value as u64 > boss_state.max_combo_seen {
            boss_state.max_combo_seen = combo_value as u64;
        }
        if sqrt_estimate >= 8 { boss_state.challenge_flag = true; }
        if sqrt_estimate < 5 { boss_state.challenge_flag = false; }

        let mut payout_amount: u64 = boss_state.base_reward + group_bonus + sqrt_estimate;
        if boss_state.challenge_flag {
            payout_amount = payout_amount + payout_amount / 2;
        }

        token::transfer(ctx.accounts.treasury_to_player_ctx(), payout_amount)?;
        boss_state.clear_count = boss_state.clear_count + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBoss<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 8 + 8 + 1)]
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
    pub player_wallet: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActDistribute<'info> {
    pub fn treasury_to_player_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let c = Transfer {
            from: self.reward_treasury.to_account_info(),
            to: self.player_wallet.to_account_info(),
            authority: self.operator.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), c)
    }
}
#[account]
pub struct BossState {
    pub operator: Pubkey,
    pub base_reward: u64,
    pub clear_count: u64,
    pub max_combo_seen: u64,
    pub challenge_flag: bool,
}
#[error_code]
pub enum RaidErr {
    #[msg("party is empty")]
    NoMembers,
}
