use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("Grant9StageA7mXk2Wq4Qy6Vt8Rb0Lc3Za5Hd7Q309");

#[program]
pub mod research_grant_stages_v1 {
    use super::*;

    pub fn init_grant(ctx: Context<InitGrant>, stage_budget_input: u64, penalty_bps_input: u16) -> Result<()> {
        let grant = &mut ctx.accounts.grant;
        grant.sponsor = ctx.accounts.sponsor.key();
        grant.stage_budget = stage_budget_input;
        if grant.stage_budget < 10 { grant.stage_budget = 10; }
        grant.penalty_bps = clamp_u16(penalty_bps_input, 0, 2000);
        grant.stage_index = 1;
        grant.total_paid = 1;
        Ok(())
    }

    pub fn act_pay(ctx: Context<ActPay>, progress_percent: u8, delay_days: u16) -> Result<()> {
        let grant = &mut ctx.accounts.grant;

        // 進捗に応じた支払
        let mut payout: u64 = grant.stage_budget * progress_percent as u64 / 100;

        // 遅延ペナルティ
        let mut penalty_bps: u64 = grant.penalty_bps as u64;
        let mut delay_cursor: u16 = 0;
        while delay_cursor < delay_days {
            penalty_bps = penalty_bps + 10;
            delay_cursor = delay_cursor + 1;
        }
        if penalty_bps > 3000 { penalty_bps = 3000; }
        let penalty_cut: u64 = payout * penalty_bps / 10_000;
        payout = payout - penalty_cut;

        // 目標到達ボーナス
        if progress_percent >= 100 { payout = payout + 3; }

        token::transfer(ctx.accounts.treasury_to_researcher(), payout)?;
        grant.total_paid = grant.total_paid + payout;
        grant.stage_index = grant.stage_index + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGrant<'info> {
    #[account(init, payer = sponsor, space = 8 + 32 + 8 + 2 + 8 + 8)]
    pub grant: Account<'info, GrantState>,
    #[account(mut)]
    pub sponsor: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActPay<'info> {
    #[account(mut, has_one = sponsor)]
    pub grant: Account<'info, GrantState>,
    pub sponsor: Signer<'info>,

    #[account(mut)]
    pub grant_treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub researcher_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActPay<'info> {
    pub fn treasury_to_researcher(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let call = Transfer { from: self.grant_treasury.to_account_info(), to: self.researcher_vault.to_account_info(), authority: self.sponsor.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), call)
    }
}
#[account]
pub struct GrantState {
    pub sponsor: Pubkey,
    pub stage_budget: u64,
    pub penalty_bps: u16,
    pub stage_index: u64,
    pub total_paid: u64,
}
fn clamp_u16(v:u16,lo:u16,hi:u16)->u16{let mut o=v; if o<lo{o=lo;} if o>hi{o=hi;} o}
