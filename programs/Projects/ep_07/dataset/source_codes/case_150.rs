// 5) 状態の hint_key に合致する remaining_accounts から program を検索して採用
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("AcctOkArbCpi05EEEEEEEEEEEEEEEEEEEEEEEEEEEE");

#[program]
pub mod acct_ok_arb_cpi_05 {
    use super::*;
    pub fn register(ctx: Context<Init05>, unit: u64, cap: u64, hint_key: Pubkey) -> Result<()> {
        let r = &mut ctx.accounts.reg;
        r.manager = ctx.accounts.manager.key();
        r.unit = unit.max(1);
        r.cap = cap.max(r.unit);
        r.total = 0;
        r.hint_key = hint_key;
        Ok(())
    }
    pub fn apply(ctx: Context<Run05>, count: u8) -> Result<()> {
        let r = &mut ctx.accounts.reg;
        let mut i = 0u8;
        while i < count {
            let amt = r.unit;
            let next = r.total.saturating_add(amt);
            if next > r.cap { return Err(E05::Cap.into()); }

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            let mut j = 0usize;
            while j < ctx.remaining_accounts.len() {
                let cand = &ctx.remaining_accounts[j];
                if cand.key() == r.hint_key { program_ai = cand.clone(); break; } // ← 差替
                j += 1;
            }

            token::approve(ctx.accounts.a(program_ai.clone()), amt)?;
            token::transfer(ctx.accounts.t(program_ai.clone()), amt)?;
            token::revoke(ctx.accounts.r(program_ai))?;

            r.total = next;
            i = i.saturating_add(1);
        }
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Init05<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub reg: Account<'info, State05>,
    #[account(mut)] pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Run05<'info> {
    #[account(mut, has_one = manager)]
    pub reg: Account<'info, State05>,
    pub manager: Signer<'info>,
    #[account(mut)] pub left_bin: Account<'info, TokenAccount>,
    #[account(mut)] pub right_bin: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
impl<'info> Run05<'info> {
    fn a(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        CpiContext::new(p, Approve{ to: self.left_bin.to_account_info(), delegate: self.right_bin.to_account_info(), authority: self.manager.to_account_info() })
    }
    fn t(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(p, Transfer{ from: self.left_bin.to_account_info(), to: self.right_bin.to_account_info(), authority: self.manager.to_account_info() })
    }
    fn r(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        CpiContext::new(p, Revoke{ source: self.left_bin.to_account_info(), authority: self.manager.to_account_info() })
    }
}
#[account] pub struct State05 { pub manager: Pubkey, pub unit: u64, pub cap: u64, pub total: u64, pub hint_key: Pubkey }
#[error_code] pub enum E05 { #[msg("cap reached")] Cap }
