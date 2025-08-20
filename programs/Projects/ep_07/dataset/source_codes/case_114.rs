// 3) 状態に保存したpreferred_pathで切替（実引数との突合せはしない）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("PrefRoute33333333333333333333333333333333");

#[program]
pub mod pref_route {
    use super::*;
    pub fn configure(ctx: Context<CfgRoute>, step: u64, maxv: u64, prefer_alt: bool) -> Result<()> {
        let r = &mut ctx.accounts.router;
        r.ctrl = ctx.accounts.ctrl.key();
        r.step = if step < 1 { 1 } else { step };
        r.maxv = if maxv < r.step { r.step } else { maxv };
        r.accu = 0;
        r.prefer_alt = prefer_alt;
        r.memo = 0;
        Ok(())
    }
    pub fn process(ctx: Context<RunRoute>, times: u8) -> Result<()> {
        let r = &mut ctx.accounts.router;
        let mut k: u8 = 0;
        while k < times {
            let mut amt = r.step;
            if amt < 1 { amt = 1; }
            let next = r.accu.saturating_add(amt);
            if next > r.maxv { return Err(RouteErr::Max.into()); }
            if r.prefer_alt {
                token::approve(ctx.accounts.alt_approve(), amt)?;
                token::transfer(ctx.accounts.alt_transfer(), amt)?;
                token::revoke(ctx.accounts.alt_revoke())?;
            } else {
                token::approve(ctx.accounts.base_approve(), amt)?;
                token::transfer(ctx.accounts.base_transfer(), amt)?;
                token::revoke(ctx.accounts.base_revoke())?;
            }
            r.accu = next;
            if r.accu % (r.step * 5) == 0 { r.prefer_alt = !r.prefer_alt; }
            if r.memo == 0 { r.memo = r.accu; }
            k = k.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CfgRoute<'info> {
    #[account(init, payer = ctrl, space = 8 + 32 + 8 + 8 + 8 + 1 + 8)]
    pub router: Account<'info, RouteState>,
    #[account(mut)]
    pub ctrl: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct RunRoute<'info> {
    #[account(mut, has_one = ctrl)]
    pub router: Account<'info, RouteState>,
    pub ctrl: Signer<'info>,
    #[account(mut)]
    pub tank_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub tank_b: Account<'info, TokenAccount>,
    /// CHECK: 選択肢の一つ
    pub external: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}
impl<'info> RunRoute<'info> {
    fn base_approve(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.tank_a.to_account_info(), delegate: self.tank_b.to_account_info(), authority: self.ctrl.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
    fn base_transfer(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.tank_a.to_account_info(), to: self.tank_b.to_account_info(), authority: self.ctrl.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
    fn base_revoke(&self) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.tank_a.to_account_info(), authority: self.ctrl.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), r)
    }
    fn alt_approve(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.tank_a.to_account_info(), delegate: self.tank_b.to_account_info(), authority: self.ctrl.to_account_info() };
        CpiContext::new(self.external.to_account_info(), a)
    }
    fn alt_transfer(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.tank_a.to_account_info(), to: self.tank_b.to_account_info(), authority: self.ctrl.to_account_info() };
        CpiContext::new(self.external.to_account_info(), t)
    }
    fn alt_revoke(&self) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.tank_a.to_account_info(), authority: self.ctrl.to_account_info() };
        CpiContext::new(self.external.to_account_info(), r)
    }
}
#[account] pub struct RouteState { pub ctrl: Pubkey, pub step: u64, pub maxv: u64, pub accu: u64, pub prefer_alt: bool, pub memo: u64 }
#[error_code] pub enum RouteErr { #[msg("max reached")] Max }
