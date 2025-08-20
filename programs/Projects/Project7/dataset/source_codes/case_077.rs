use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Token, TokenAccount, Mint};

declare_id!("Fit1StepRwrd9Qv4Lk7Zp2A1Xt6Uy8Nc3Hb5Dq901");

#[program]
pub mod fitness_steps_reward_v1 {
    use super::*;

    pub fn init_plan(ctx: Context<InitPlan>, daily_cap_input: u64, base_point_input: u64, mult_bps_input: u16) -> Result<()> {
        let plan = &mut ctx.accounts.plan;
        plan.operator = ctx.accounts.operator.key();
        plan.daily_cap = daily_cap_input;
        if plan.daily_cap < 20 { plan.daily_cap = 20; }
        plan.base_point = base_point_input;
        if plan.base_point < 1 { plan.base_point = 1; }
        plan.multiplier_bps = clamp_u16(mult_bps_input, 0, 3000);
        plan.issued_today = 0;
        plan.streak_days = 1;
        Ok(())
    }

    pub fn act_award(ctx: Context<ActAward>, steps: u64, streak_flag: bool) -> Result<()> {
        let plan = &mut ctx.accounts.plan;

        // 1000歩ごとに段階加点
        let mut scaled = plan.base_point;
        let mut walked = steps;
        while walked >= 1000 {
            scaled = scaled + 1 + (walked % 3000) / 1000;
            if walked >= 1000 { walked = walked - 1000; } else { walked = 0; }
        }

        // ストリーク倍率
        let mut bps = plan.multiplier_bps as u64;
        if streak_flag { bps = bps + 150; }
        if plan.streak_days % 7 == 0 { bps = bps + 200; }

        let boosted = scaled + (scaled * bps / 10_000);
        let projected = plan.issued_today + boosted;

        if projected > plan.daily_cap {
            let rest = plan.daily_cap - plan.issued_today;
            if rest > 0 { token::mint_to(ctx.accounts.mint_ctx(), rest)?; }
            plan.issued_today = plan.daily_cap;
            plan.streak_days = 1;
            return Err(FitErr::DailyCap.into());
        }

        token::mint_to(ctx.accounts.mint_ctx(), boosted)?;
        plan.issued_today = projected;
        plan.streak_days = plan.streak_days + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPlan<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 8 + 2 + 8 + 8)]
    pub plan: Account<'info, FitnessPlan>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActAward<'info> {
    #[account(mut, has_one = operator)]
    pub plan: Account<'info, FitnessPlan>,
    pub operator: Signer<'info>,

    pub point_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_point_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActAward<'info> {
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let c = MintTo {
            mint: self.point_mint.to_account_info(),
            to: self.user_point_vault.to_account_info(),
            authority: self.operator.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), c)
    }
}
#[account]
pub struct FitnessPlan {
    pub operator: Pubkey,
    pub daily_cap: u64,
    pub base_point: u64,
    pub multiplier_bps: u16,
    pub issued_today: u64,
    pub streak_days: u64,
}
#[error_code]
pub enum FitErr { #[msg("daily cap reached")] DailyCap }

fn clamp_u16(v:u16, lo:u16, hi:u16)->u16{ let mut o=v; if o<lo{o=lo;} if o>hi{o=hi;} o }
