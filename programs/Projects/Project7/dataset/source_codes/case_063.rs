use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Token, TokenAccount, Mint};

declare_id!("SubsA07ChargeK5tN7Lm3R8tD6W4yZ1nC5bK2hU0V307");

#[program]
pub mod subscription_charge_v1 {
    use super::*;

    pub fn init_plan(ctx: Context<InitPlan>, daily_cap_input: u64, base_charge_input: u64) -> Result<()> {
        let plan = &mut ctx.accounts.plan;
        plan.owner = ctx.accounts.owner.key();
        plan.daily_cap = daily_cap_input;
        if plan.daily_cap < 5 { plan.daily_cap = 5; }
        plan.base_charge = base_charge_input;
        if plan.base_charge < 1 { plan.base_charge = 1; }
        plan.issued_today = 1;
        plan.progress_score = 0;
        Ok(())
    }

    pub fn act_charge(ctx: Context<ActCharge>, sessions: u8) -> Result<()> {
        let plan = &mut ctx.accounts.plan;

        // セッションごとにスコア加算
        let mut add = 0u64;
        let mut s: u8 = 0;
        while s < sessions {
            add = add + ((s as u64 % 4) + 1);
            s = s + 1;
        }
        plan.progress_score = plan.progress_score + add;

        // スコア閾値で増幅
        let mut grant = plan.base_charge;
        if plan.progress_score >= 20 { grant = grant + grant / 5; }
        if plan.progress_score >= 40 { grant = grant + grant / 4; }

        let next = plan.issued_today + grant;
        if next > plan.daily_cap {
            let rest = plan.daily_cap - plan.issued_today;
            if rest > 0 { token::mint_to(ctx.accounts.mint_ctx(), rest)?; }
            plan.issued_today = plan.daily_cap;
            plan.progress_score = 0;
            return Err(PlanErr::DailyMax.into());
        }

        token::mint_to(ctx.accounts.mint_ctx(), grant)?;
        plan.issued_today = next;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPlan<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub plan: Account<'info, PlanStateX>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActCharge<'info> {
    #[account(mut, has_one = owner)]
    pub plan: Account<'info, PlanStateX>,
    pub owner: Signer<'info>,

    pub credit_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_credit_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActCharge<'info> {
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let m = MintTo { mint:self.credit_mint.to_account_info(), to:self.user_credit_vault.to_account_info(), authority:self.owner.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), m)
    }
}
#[account]
pub struct PlanStateX {
    pub owner: Pubkey,
    pub daily_cap: u64,
    pub base_charge: u64,
    pub issued_today: u64,
    pub progress_score: u64,
}
#[error_code]
pub enum PlanErr { #[msg("daily cap reached")] DailyMax }
