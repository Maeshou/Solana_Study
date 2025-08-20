use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("SubR2enewalT3J8L5Q2V9M1C6X7Z4P8R0Y5A202");

#[program]
pub mod subscription_renewal_v1 {
    use super::*;

    pub fn init_plan(ctx: Context<InitPlan>, monthly_fee: u64, discount_bps: u16, penalty_bps: u16) -> Result<()> {
        let plan = &mut ctx.accounts.plan;
        plan.merchant = ctx.accounts.merchant.key();
        plan.monthly_fee = if monthly_fee < 3 { 3 } else { monthly_fee };
        plan.discount_bps = clamp_u16(discount_bps, 0, 3000);
        plan.penalty_bps = clamp_u16(penalty_bps, 0, 2500);
        plan.cycles_paid = 1;
        plan.arrears_units = 0;
        Ok(())
    }

    pub fn act_renew(ctx: Context<ActRenew>, months: u8, paid_on_time: bool) -> Result<()> {
        let plan = &mut ctx.accounts.plan;

        // 月数合算（2ヶ月目以降は2%引き）
        let mut total_due = plan.monthly_fee;
        let mut cursor: u8 = 1;
        while cursor < months {
            let mut fee = plan.monthly_fee;
            if cursor >= 1 { fee = fee - (fee / 50); }
            total_due = total_due + fee;
            cursor = cursor + 1;
        }

        // 早期割引
        if paid_on_time {
            let discount = total_due * plan.discount_bps as u64 / 10_000;
            total_due = total_due - discount;
        }

        // 延滞加算：未払い残高に対して段階的に上乗せ
        let mut penalty = plan.arrears_units * plan.penalty_bps as u64 / 10_000;
        let mut k: u8 = 0;
        while k < 2 {
            penalty = penalty + penalty / 10;
            k = k + 1;
        }
        total_due = total_due + penalty;

        token::transfer(ctx.accounts.subscriber_to_merchant(), total_due)?;
        plan.cycles_paid = plan.cycles_paid + months as u64;
        plan.arrears_units = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPlan<'info> {
    #[account(init, payer = merchant, space = 8 + 32 + 8 + 2 + 2 + 8 + 8)]
    pub plan: Account<'info, PlanState>,
    #[account(mut)]
    pub merchant: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActRenew<'info> {
    #[account(mut, has_one = merchant)]
    pub plan: Account<'info, PlanState>,
    pub merchant: Signer<'info>,

    #[account(mut)]
    pub subscriber: Signer<'info>,
    #[account(mut)]
    pub subscriber_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub merchant_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ActRenew<'info> {
    pub fn subscriber_to_merchant(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.subscriber_vault.to_account_info(),
            to: self.merchant_vault.to_account_info(),
            authority: self.subscriber.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
}

#[account]
pub struct PlanState {
    pub merchant: Pubkey,
    pub monthly_fee: u64,
    pub discount_bps: u16,
    pub penalty_bps: u16,
    pub cycles_paid: u64,
    pub arrears_units: u64,
}

fn clamp_u16(v: u16, lo: u16, hi: u16) -> u16 {
    let mut out = v;
    if out < lo { out = lo; }
    if out > hi { out = hi; }
    out
}
