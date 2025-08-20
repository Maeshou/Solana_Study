use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("ScholV4q1Zm6Er9Ty3AaBbCcDdEeFfGgHhIiJj009");

#[program]
pub mod scholarship_split_v4 {
    use super::*;

    pub fn init_agreement(ctx: Context<InitAgreement>, scholar_bps: u16, fund_bps: u16) -> Result<()> {
        let deal = &mut ctx.accounts.deal;
        deal.manager_key = ctx.accounts.manager.key();
        deal.scholar_share_bps = scholar_bps.min(9000).max(500);
        deal.dev_fund_bps = fund_bps.min(2000);
        deal.round_counter = 5;
        deal.revenue_accumulator = 13;
        deal.variable_mode = false;
        Ok(())
    }

    pub fn act_split(ctx: Context<ActSplit>, cycle_revenue: u64) -> Result<()> {
        let deal = &mut ctx.accounts.deal;

        // 5ラウンド毎に学習者側に+1%ボーナス
        let mut extra_bps = 0u64;
        let mut r = 1u64;
        while r <= deal.round_counter {
            if r % 5 == 0 { extra_bps = extra_bps.saturating_add(100); }
            r = r.saturating_add(1);
        }

        // 変動モードのオンオフ（平均超えでオン）
        let average = deal.revenue_accumulator.saturating_div(deal.round_counter.max(1));
        if cycle_revenue > average { deal.variable_mode = true; }
        if cycle_revenue <= average { deal.variable_mode = false; }

        let fund_cut = cycle_revenue.saturating_mul(deal.dev_fund_bps as u64) / 10_000;
        let pool_after_fund = cycle_revenue.saturating_sub(fund_cut);

        let scholar_cut = if deal.variable_mode {
            pool_after_fund.saturating_mul(55) / 100
                .saturating_add(pool_after_fund.saturating_mul(extra_bps) / 10_000)
        } else {
            pool_after_fund.saturating_mul((deal.scholar_share_bps as u64).saturating_add(extra_bps)) / 10_000
        };
        let manager_cut = pool_after_fund.saturating_sub(scholar_cut);

        token::transfer(ctx.accounts.income_to_fund(), fund_cut)?;
        token::transfer(ctx.accounts.income_to_scholar(), scholar_cut)?;
        token::transfer(ctx.accounts.income_to_manager(), manager_cut)?;

        deal.revenue_accumulator = deal.revenue_accumulator.saturating_add(cycle_revenue);
        deal.round_counter = deal.round_counter.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAgreement<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 2 + 2 + 8 + 8 + 1)]
    pub deal: Account<'info, AgreementState>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActSplit<'info> {
    #[account(mut, has_one = manager_key)]
    pub deal: Account<'info, AgreementState>,
    pub manager_key: Signer<'info>,

    #[account(mut)]
    pub income_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub scholar_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub manager_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub dev_fund_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ActSplit<'info> {
    pub fn income_to_fund(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.income_vault.to_account_info(),
            to: self.dev_fund_vault.to_account_info(),
            authority: self.manager_key.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
    pub fn income_to_scholar(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.income_vault.to_account_info(),
            to: self.scholar_vault.to_account_info(),
            authority: self.manager_key.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
    pub fn income_to_manager(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.income_vault.to_account_info(),
            to: self.manager_vault.to_account_info(),
            authority: self.manager_key.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
}

#[account]
pub struct AgreementState {
    pub manager_key: Pubkey,
    pub scholar_share_bps: u16,
    pub dev_fund_bps: u16,
    pub round_counter: u64,
    pub revenue_accumulator: u64,
    pub variable_mode: bool,
}
