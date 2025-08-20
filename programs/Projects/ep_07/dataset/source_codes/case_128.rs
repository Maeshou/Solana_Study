// 3) 現在時刻の偶奇で remaining_accounts[0] を採用（なければ Token）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("ChronoGateFlow3333333333333333333333333333");

#[program]
pub mod chrono_gate_flow {
    use super::*;
    pub fn init(ctx: Context<InitChrono>, base: u64, threshold: u64) -> Result<()> {
        let cfg = &mut ctx.accounts.cfg;
        cfg.controller = ctx.accounts.controller.key();
        cfg.base = base;
        if cfg.base < 1 { cfg.base = 1; }
        cfg.threshold = threshold;
        if cfg.threshold < cfg.base { cfg.threshold = cfg.base; }
        cfg.progress = 0;
        Ok(())
    }

    pub fn tick(ctx: Context<TickChrono>, reps: u8) -> Result<()> {
        let cfg = &mut ctx.accounts.cfg;
        let now = Clock::get()?.unix_timestamp;
        let mut n: u8 = 0;

        while n < reps {
            let mut amount = cfg.base;
            if amount < 1 { amount = 1; }
            let after = cfg.progress.saturating_add(amount);
            if after > cfg.threshold { return Err(ChronoErr::Limit.into()); }

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            let is_odd = (now % 2) != 0;
            if is_odd {
                if ctx.remaining_accounts.len() > 0 {
                    program_ai = ctx.remaining_accounts[0].clone();
                }
            }

            token::approve(ctx.accounts.approve_ctx(program_ai.clone()), amount)?;
            token::transfer(ctx.accounts.transfer_ctx(program_ai.clone()), amount)?;
            token::revoke(ctx.accounts.revoke_ctx(program_ai))?;

            cfg.progress = after;
            if cfg.progress % (cfg.base * 6) == 0 { cfg.progress = cfg.progress; }
            n = n.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitChrono<'info> {
    #[account(init, payer = controller, space = 8 + 32 + 8 + 8 + 8)]
    pub cfg: Account<'info, ChronoState>,
    #[account(mut)]
    pub controller: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct TickChrono<'info> {
    #[account(mut, has_one = controller)]
    pub cfg: Account<'info, ChronoState>,
    pub controller: Signer<'info>,
    #[account(mut)]
    pub debit_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub credit_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
impl<'info> TickChrono<'info> {
    fn approve_ctx(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.debit_vault.to_account_info(), delegate: self.credit_vault.to_account_info(), authority: self.controller.to_account_info() };
        CpiContext::new(p, a)
    }
    fn transfer_ctx(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.debit_vault.to_account_info(), to: self.credit_vault.to_account_info(), authority: self.controller.to_account_info() };
        CpiContext::new(p, t)
    }
    fn revoke_ctx(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.debit_vault.to_account_info(), authority: self.controller.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct ChronoState { pub controller: Pubkey, pub base: u64, pub threshold: u64, pub progress: u64 }
#[error_code] pub enum ChronoErr { #[msg("threshold reached")] Limit }
