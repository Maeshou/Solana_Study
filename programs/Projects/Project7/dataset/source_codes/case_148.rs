// 4) 承認→中継→最終転送の3段階で、段ごとに同じ可変programを再利用
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("RelayFlow44444444444444444444444444444444");

#[program]
pub mod relay_flow {
    use super::*;
    pub fn init(ctx: Context<InitRelay>, q: u64, limit: u64) -> Result<()> {
        let s = &mut ctx.accounts.stage;
        s.keyholder = ctx.accounts.keyholder.key();
        s.q = if q < 1 { 1 } else { q };
        s.limit = if limit < s.q { s.q } else { limit };
        s.done = 0;
        s.toggle = 0;
        Ok(())
    }
    pub fn execute(ctx: Context<ExecRelay>, rounds: u8) -> Result<()> {
        let s = &mut ctx.accounts.stage;
        let mut c: u8 = 0;
        while c < rounds {
            let mut amt = s.q;
            if amt < 1 { amt = 1; }
            let nxt = s.done.saturating_add(amt);
            if nxt > s.limit { return Err(RelayErr::Bound.into()); }
            let choose_alt = (s.toggle % 2) == 1;
            let prg = if choose_alt { ctx.accounts.flex.to_account_info() } else { ctx.accounts.token_program.to_account_info() };

            token::approve(ctx.accounts.ctx_approve(prg.clone()), amt)?;
            // 中継：A -> buffer
            token::transfer(ctx.accounts.ctx_transfer(prg.clone(), true), amt)?;
            // 最終：buffer -> B
            token::transfer(ctx.accounts.ctx_transfer(prg.clone(), false), amt)?;
            token::revoke(ctx.accounts.ctx_revoke(prg))?;

            s.done = nxt;
            if s.done % (s.q * 2) == 0 { s.toggle = s.toggle.saturating_add(1); }
            c = c.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRelay<'info> {
    #[account(init, payer = keyholder, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub stage: Account<'info, RelayState>,
    #[account(mut)]
    pub keyholder: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ExecRelay<'info> {
    #[account(mut, has_one = keyholder)]
    pub stage: Account<'info, RelayState>,
    pub keyholder: Signer<'info>,
    #[account(mut)]
    pub vault_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault_b: Account<'info, TokenAccount>,
    #[account(mut)]
    pub buffer_vault: Account<'info, TokenAccount>,
    /// CHECK: 切替用
    pub flex: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}
impl<'info> ExecRelay<'info> {
    fn ctx_approve(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.vault_a.to_account_info(), delegate: self.buffer_vault.to_account_info(), authority: self.keyholder.to_account_info() };
        CpiContext::new(p, a)
    }
    fn ctx_transfer(&self, p: AccountInfo<'info>, to_buffer: bool) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let from_ai = if to_buffer { self.vault_a.to_account_info() } else { self.buffer_vault.to_account_info() };
        let to_ai   = if to_buffer { self.buffer_vault.to_account_info() } else { self.vault_b.to_account_info() };
        let t = Transfer { from: from_ai, to: to_ai, authority: self.keyholder.to_account_info() };
        CpiContext::new(p, t)
    }
    fn ctx_revoke(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.vault_a.to_account_info(), authority: self.keyholder.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct RelayState { pub keyholder: Pubkey, pub q: u64, pub limit: u64, pub done: u64, pub toggle: u64 }
#[error_code] pub enum RelayErr { #[msg("bound exceeded")] Bound }
