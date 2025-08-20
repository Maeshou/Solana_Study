// 2) Clockの偶奇で選択（時間により program を切替）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("TimeGate2222222222222222222222222222222222");

#[program]
pub mod time_gate {
    use super::*;
    pub fn init(ctx: Context<InitTime>, base: u64, ceiling: u64) -> Result<()> {
        let g = &mut ctx.accounts.gate;
        g.admin = ctx.accounts.admin.key();
        g.base = if base < 1 { 1 } else { base };
        g.ceiling = if ceiling < g.base { g.base } else { ceiling };
        g.count = 0;
        Ok(())
    }
    pub fn tick(ctx: Context<TickTime>, steps: u8) -> Result<()> {
        let g = &mut ctx.accounts.gate;
        let now = Clock::get()?.unix_timestamp;
        let mut i: u8 = 0;
        while i < steps {
            let mut size = g.base;
            if size < 1 { size = 1; }
            let next = g.count.saturating_add(size);
            if next > g.ceiling { return Err(TGateErr::Ceil.into()); }
            let use_alt = (now % 2) != 0;
            let prg = if use_alt {
                ctx.accounts.plugin.to_account_info()
            } else {
                ctx.accounts.token_program.to_account_info()
            };
            token::approve(ctx.accounts.approve_with(prg.clone()), size)?;
            token::transfer(ctx.accounts.transfer_with(prg.clone()), size)?;
            token::revoke(ctx.accounts.revoke_with(prg))?;
            g.count = next;
            if g.count % (g.base * 4) == 0 { g.count = g.count; }
            i = i.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTime<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8 + 8)]
    pub gate: Account<'info, TimeState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct TickTime<'info> {
    #[account(mut, has_one = admin)]
    pub gate: Account<'info, TimeState>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub src: Account<'info, TokenAccount>,
    #[account(mut)]
    pub dst: Account<'info, TokenAccount>,
    /// CHECK: 時刻で切替対象
    pub plugin: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}
impl<'info> TickTime<'info> {
    fn approve_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.src.to_account_info(), delegate: self.dst.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(p, a)
    }
    fn transfer_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.src.to_account_info(), to: self.dst.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(p, t)
    }
    fn revoke_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.src.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct TimeState { pub admin: Pubkey, pub base: u64, pub ceiling: u64, pub count: u64 }
#[error_code] pub enum TGateErr { #[msg("limit hit")] Ceil }
