use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("ScholV3e1dZ0ScholV3e1dZ0ScholV3e1dZ0Sc10");

#[program]
pub mod scholarship_split_v3 {
    use super::*;

    pub fn init_deal(ctx: Context<InitDeal>, base_bps: u16) -> Result<()> {
        let d = &mut ctx.accounts.deal;
        d.manager = ctx.accounts.manager.key();
        d.scholar_base_bps = base_bps.min(9000).max(500);
        d.round_index = 4;
        d.total_revenue_seen = 11;
        d.dynamic_mode = false;
        Ok(())
    }

    pub fn act_split(ctx: Context<ActSplit>, revenue: u64) -> Result<()> {
        let d = &mut ctx.accounts.deal;

        // ラウンドごとの固定ボーナス（5ラウンドごとに+1%）
        let mut add_bps = 0u64;
        let mut i = 1u64;
        while i <= d.round_index {
            if i % 5 == 0 { add_bps = add_bps.saturating_add(100); }
            i = i.saturating_add(1);
        }

        // 動的モード切替：収益が過去平均を上回ると有効化
        let avg = d.total_revenue_seen.saturating_div(d.round_index.max(1));
        if revenue > avg { d.dynamic_mode = true; }
        if revenue <= avg { d.dynamic_mode = false; }

        let scholar_cut = if d.dynamic_mode {
            // 動的：60% + ボーナスbps
            let base = revenue.saturating_mul(60) / 100;
            base.saturating_add(revenue.saturating_mul(add_bps) / 10_000)
        } else {
            revenue.saturating_mul((d.scholar_base_bps as u64).saturating_add(add_bps)) / 10_000
        };
        let manager_cut = revenue.saturating_sub(scholar_cut);

        token::transfer(ctx.accounts.revenue_to_scholar(), scholar_cut)?;
        token::transfer(ctx.accounts.revenue_to_manager(), manager_cut)?;

        d.total_revenue_seen = d.total_revenue_seen.saturating_add(revenue);
        d.round_index = d.round_index.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitDeal<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 2 + 8 + 8 + 1)]
    pub deal: Account<'info, DealState>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActSplit<'info> {
    #[account(mut, has_one = manager)]
    pub deal: Account<'info, DealState>,
    pub manager: Signer<'info>,
    #[account(mut)]
    pub revenue_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub scholar_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub manager_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ActSplit<'info> {
    pub fn revenue_to_scholar(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let a = Transfer {
            from: self.revenue_vault.to_account_info(),
            to: self.scholar_vault.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
    pub fn revenue_to_manager(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let a = Transfer {
            from: self.revenue_vault.to_account_info(),
            to: self.manager_vault.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
}

#[account]
pub struct DealState {
    pub manager: Pubkey,
    pub scholar_base_bps: u16,
    pub round_index: u64,
    pub total_revenue_seen: u64,
    pub dynamic_mode: bool,
}
