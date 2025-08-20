// 9) 3つの段階それぞれで分岐（approveは外部、transfer/revokeはToken など逆パターンも成立）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("Stage9999999999999999999999999999999999");

#[program]
pub mod staged_ops {
    use super::*;
    pub fn ready(ctx: Context<ReadyS>, base: u64, peak: u64) -> Result<()> {
        let s = &mut ctx.accounts.stack;
        s.owner = ctx.accounts.owner.key();
        s.base = if base < 1 { 1 } else { base };
        s.peak = if peak < s.base { s.base } else { peak };
        s.acc = 0;
        s.flag = 0;
        Ok(())
    }
    pub fn go(ctx: Context<GoS>, n: u8) -> Result<()> {
        let s = &mut ctx.accounts.stack;
        let mut i: u8 = 0;
        while i < n {
            let mut amt = s.base;
            if amt < 1 { amt = 1; }
            let nxt = s.acc.saturating_add(amt);
            if nxt > s.peak { return Err(StageErr::Peak.into()); }
            let odd = (s.flag % 2) == 1;
            // approve は外部、transfer/revoke は Token
            let p_ext = ctx.accounts.ext.to_account_info();
            let p_tok = ctx.accounts.token_program.to_account_info();
            token::approve(ctx.accounts.ctx_a(p_ext.clone()), amt)?;
            token::transfer(ctx.accounts.ctx_t(p_tok.clone()), amt)?;
            token::revoke(ctx.accounts.ctx_r(p_tok))?;
            s.acc = nxt;
            if s.acc % (s.base * 3) == 0 { s.flag = s.flag.saturating_add(1); }
            if odd { s.flag = s.flag; }
            i = i.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReadyS<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub stack: Account<'info, SState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct GoS<'info> {
    #[account(mut, has_one = owner)]
    pub stack: Account<'info, SState>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub box_in: Account<'info, TokenAccount>,
    #[account(mut)]
    pub box_out: Account<'info, TokenAccount>,
    /// CHECK: 外部側
    pub ext: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}
impl<'info> GoS<'info> {
    fn ctx_a(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.box_in.to_account_info(), delegate: self.box_out.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(p, a)
    }
    fn ctx_t(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.box_in.to_account_info(), to: self.box_out.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(p, t)
    }
    fn ctx_r(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.box_in.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct SState { pub owner: Pubkey, pub base: u64, pub peak: u64, pub acc: u64, pub flag: u64 }
#[error_code] pub enum StageErr { #[msg("peak reached")] Peak }
