// 10) ウィンドウ内の位置に応じて前半は Token、後半は AccountInfo 経路を採用（窓リセットあり）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("WindowPhaseAAAAABBBBBCCCCCDDDDDEEEEEFFFFF");

#[program]
pub mod window_phase_selector {
    use super::*;
    pub fn open(ctx: Context<OpenWindow>, step: u64, cap: u64, window_size: u64) -> Result<()> {
        let w = &mut ctx.accounts.window;
        w.guard = ctx.accounts.guard.key();
        w.step = step;
        if w.step < 1 { w.step = 1; }
        w.cap = cap;
        if w.cap < w.step { w.cap = w.step; }
        w.window_size = if window_size < 2 { 2 } else { window_size };
        w.position = 0;
        w.total = 0;
        Ok(())
    }

    pub fn pump(ctx: Context<PumpWindow>, count: u8) -> Result<()> {
        let w = &mut ctx.accounts.window;
        let mut i: u8 = 0;
        while i < count {
            let mut amount = w.step;
            if amount < 1 { amount = 1; }
            let next = w.total.saturating_add(amount);
            if next > w.cap { return Err(WindowErr::Cap.into()); }

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            let half = w.window_size / 2;
            if w.position >= half { program_ai = ctx.accounts.gateway.clone(); }

            token::approve(ctx.accounts.ap(program_ai.clone()), amount)?;
            token::transfer(ctx.accounts.tr(program_ai.clone()), amount)?;
            token::revoke(ctx.accounts.rv(program_ai))?;

            w.total = next;
            w.position = w.position.saturating_add(1);
            if w.position % w.window_size == 0 { w.position = 0; }
            i = i.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct OpenWindow<'info> {
    #[account(init, payer = guard, space = 8 + 32 + 8 + 8 + 8 + 8 + 8)]
    pub window: Account<'info, WindowState>,
    #[account(mut)] pub guard: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct PumpWindow<'info> {
    #[account(mut, has_one = guard)]
    pub window: Account<'info, WindowState>,
    pub guard: Signer<'info>,
    #[account(mut)] pub inlet: Account<'info, TokenAccount>,
    #[account(mut)] pub outlet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub gateway: AccountInfo<'info>,
}
impl<'info> PumpWindow<'info> {
    fn ap(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.inlet.to_account_info(), delegate: self.outlet.to_account_info(), authority: self.guard.to_account_info() };
        CpiContext::new(p, a)
    }
    fn tr(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.inlet.to_account_info(), to: self.outlet.to_account_info(), authority: self.guard.to_account_info() };
        CpiContext::new(p, t)
    }
    fn rv(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.inlet.to_account_info(), authority: self.guard.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct WindowState { pub guard: Pubkey, pub step: u64, pub cap: u64, pub window_size: u64, pub position: u64, pub total: u64 }
#[error_code] pub enum WindowErr { #[msg("cap reached")] Cap }
