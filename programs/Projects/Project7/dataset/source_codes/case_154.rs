// 10) 期間ごとの回数窓で切替（ウィンドウ内で一定回数を越えると alt を使用）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("WindowAAAAAAAABBBBBBBBBBCCCCCCCCCCDDDDDD");

#[program]
pub mod window_router {
    use super::*;
    pub fn open(ctx: Context<OpenW>, unit: u64, roof: u64, window: u64) -> Result<()> {
        let w = &mut ctx.accounts.wcfg;
        w.guard = ctx.accounts.guard.key();
        w.unit = if unit < 1 { 1 } else { unit };
        w.roof = if roof < w.unit { w.unit } else { roof };
        w.window = if window < 1 { 1 } else { window };
        w.total = 0;
        w.in_window = 0;
        Ok(())
    }
    pub fn pump(ctx: Context<PumpW>, n: u8) -> Result<()> {
        let w = &mut ctx.accounts.wcfg;
        let mut r: u8 = 0;
        while r < n {
            let mut amt = w.unit;
            if amt < 1 { amt = 1; }
            let nxt = w.total.saturating_add(amt);
            if nxt > w.roof { return Err(WinErr::Roof.into()); }

            let over = w.in_window >= w.window;
            let prg = if over { ctx.accounts.gateway.to_account_info() } else { ctx.accounts.token_program.to_account_info() };
            token::approve(ctx.accounts.a(prg.clone()), amt)?;
            token::transfer(ctx.accounts.t(prg.clone()), amt)?;
            token::revoke(ctx.accounts.r(prg))?;

            w.total = nxt;
            w.in_window = w.in_window.saturating_add(1);
            if w.in_window % (w.window * 2) == 0 { w.in_window = 0; }
            r = r.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct OpenW<'info> {
    #[account(init, payer = guard, space = 8 + 32 + 8 + 8 + 8 + 8 + 8)]
    pub wcfg: Account<'info, WinState>,
    #[account(mut)]
    pub guard: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct PumpW<'info> {
    #[account(mut, has_one = guard)]
    pub wcfg: Account<'info, WinState>,
    pub guard: Signer<'info>,
    #[account(mut)]
    pub tube_in: Account<'info, TokenAccount>,
    #[account(mut)]
    pub tube_out: Account<'info, TokenAccount>,
    /// CHECK: ウィンドウ超過時に使用
    pub gateway: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}
impl<'info> PumpW<'info> {
    fn a(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.tube_in.to_account_info(), delegate: self.tube_out.to_account_info(), authority: self.guard.to_account_info() };
        CpiContext::new(p, a)
    }
    fn t(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.tube_in.to_account_info(), to: self.tube_out.to_account_info(), authority: self.guard.to_account_info() };
        CpiContext::new(p, t)
    }
    fn r(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.tube_in.to_account_info(), authority: self.guard.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct WinState { pub guard: Pubkey, pub unit: u64, pub roof: u64, pub window: u64, pub total: u64, pub in_window: u64 }
#[error_code] pub enum WinErr { #[msg("roof reached")] Roof }
