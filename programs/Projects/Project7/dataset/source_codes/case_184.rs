// 6) ウィンドウ後半のみ AccountInfo 経路に切替（position >= half）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("AcctOkArbCpi06FFFFFFFFFFFFFFFFFFFFFFFFFFFF");

#[program]
pub mod acct_ok_arb_cpi_06 {
    use super::*;
    pub fn open(ctx: Context<Init06>, step: u64, cap: u64, window: u64) -> Result<()> {
        let w = &mut ctx.accounts.window;
        w.owner = ctx.accounts.owner.key();
        w.step = step.max(1);
        w.cap = cap.max(w.step);
        w.window = window.max(2);
        w.pos = 0;
        w.total = 0;
        Ok(())
    }
    pub fn pump(ctx: Context<Run06>, times: u8) -> Result<()> {
        let w = &mut ctx.accounts.window;
        let mut t = 0u8;
        while t < times {
            let amt = w.step;
            let next = w.total.saturating_add(amt);
            if next > w.cap { return Err(E06::Cap.into()); }

            let half = w.window / 2;
            let mut program_ai = ctx.accounts.token_program.to_account_info();
            if w.pos >= half { program_ai = ctx.accounts.gateway.clone(); } // ← 差替

            token::approve(ctx.accounts.ap(program_ai.clone()), amt)?;
            token::transfer(ctx.accounts.tr(program_ai.clone()), amt)?;
            token::revoke(ctx.accounts.rv(program_ai))?;

            w.total = next;
            w.pos = w.pos.saturating_add(1);
            if w.pos % w.window == 0 { w.pos = 0; }
            t = t.saturating_add(1);
        }
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Init06<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 8 + 8)]
    pub window: Account<'info, State06>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Run06<'info> {
    #[account(mut, has_one = owner)]
    pub window: Account<'info, State06>,
    pub owner: Signer<'info>,
    #[account(mut)] pub inlet: Account<'info, TokenAccount>,
    #[account(mut)] pub outlet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub gateway: AccountInfo<'info>,                         // ← AccountInfo
}
impl<'info> Run06<'info> {
    fn ap(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        CpiContext::new(p, Approve{ to: self.inlet.to_account_info(), delegate: self.outlet.to_account_info(), authority: self.owner.to_account_info() })
    }
    fn tr(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(p, Transfer{ from: self.inlet.to_account_info(), to: self.outlet.to_account_info(), authority: self.owner.to_account_info() })
    }
    fn rv(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        CpiContext::new(p, Revoke{ source: self.inlet.to_account_info(), authority: self.owner.to_account_info() })
    }
}
#[account] pub struct State06 { pub owner: Pubkey, pub step: u64, pub cap: u64, pub window: u64, pub pos: u64, pub total: u64 }
#[error_code] pub enum E06 { #[msg("cap reached")] Cap }
