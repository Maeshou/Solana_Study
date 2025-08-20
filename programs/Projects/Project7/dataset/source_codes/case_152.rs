// 8) ランタイムカウンタの偶奇で alt を使用（単一ifのみ）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("Parity8888888888888888888888888888888888");

#[program]
pub mod parity_path {
    use super::*;
    pub fn seed(ctx: Context<SeedP>, base: u64, lid: u64) -> Result<()> {
        let p = &mut ctx.accounts.panel;
        p.admin = ctx.accounts.admin.key();
        p.base = if base < 1 { 1 } else { base };
        p.lid = if lid < p.base { p.base } else { lid };
        p.ct = 0;
        Ok(())
    }
    pub fn fire(ctx: Context<FireP>, times: u8) -> Result<()> {
        let p = &mut ctx.accounts.panel;
        let mut rounds: u8 = 0;
        while rounds < times {
            let mut amt = p.base;
            if amt < 1 { amt = 1; }
            let plan = p.ct.saturating_add(amt);
            if plan > p.lid { return Err(ParityErr::Overflow.into()); }
            let odd = (plan % 2) == 1;
            let prg = if odd { ctx.accounts.switcher.to_account_info() } else { ctx.accounts.token_program.to_account_info() };
            token::approve(ctx.accounts.ctx_a(prg.clone()), amt)?;
            token::transfer(ctx.accounts.ctx_t(prg.clone()), amt)?;
            token::revoke(ctx.accounts.ctx_r(prg))?;
            p.ct = plan;
            if p.ct % (p.base * 8) == 0 { p.ct = p.ct; }
            rounds = rounds.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SeedP<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8 + 8)]
    pub panel: Account<'info, PState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct FireP<'info> {
    #[account(mut, has_one = admin)]
    pub panel: Account<'info, PState>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub left: Account<'info, TokenAccount>,
    #[account(mut)]
    pub right: Account<'info, TokenAccount>,
    /// CHECK: 偶奇で切替
    pub switcher: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}
impl<'info> FireP<'info> {
    fn ctx_a(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.left.to_account_info(), delegate: self.right.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(p, a)
    }
    fn ctx_t(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.left.to_account_info(), to: self.right.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(p, t)
    }
    fn ctx_r(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.left.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct PState { pub admin: Pubkey, pub base: u64, pub lid: u64, pub ct: u64 }
#[error_code] pub enum ParityErr { #[msg("overflow")] Overflow }
