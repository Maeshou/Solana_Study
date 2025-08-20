// 3) 実行引数 prefer_external による切替（AccountInfo フィールドを直に採用）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("AcctOkArbCpi03CCCCCCCCCCCCCCCCCCCCCCCCCCCC");

#[program]
pub mod acct_ok_arb_cpi_03 {
    use super::*;
    pub fn setup(ctx: Context<Init03>, step: u64, cap: u64) -> Result<()> {
        let s = &mut ctx.accounts.set;
        s.supervisor = ctx.accounts.supervisor.key();
        s.step = step.max(1);
        s.cap = cap.max(s.step);
        s.total = 0;
        Ok(())
    }
    pub fn perform(ctx: Context<Run03>, rounds: u8, prefer_external: bool) -> Result<()> {
        let s = &mut ctx.accounts.set;
        let mut i = 0u8;
        while i < rounds {
            let amt = s.step;
            let next = s.total.saturating_add(amt);
            if next > s.cap { return Err(E03::Cap.into()); }

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            if prefer_external { program_ai = ctx.accounts.external_program.clone(); } // ← 差替

            token::approve(ctx.accounts.ap(program_ai.clone()), amt)?;
            token::transfer(ctx.accounts.tr(program_ai.clone()), amt)?;
            token::revoke(ctx.accounts.rv(program_ai))?;

            s.total = next;
            i = i.saturating_add(1);
        }
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Init03<'info> {
    #[account(init, payer = supervisor, space = 8 + 32 + 8 + 8 + 8)]
    pub set: Account<'info, State03>,
    #[account(mut)] pub supervisor: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Run03<'info> {
    #[account(mut, has_one = supervisor)]
    pub set: Account<'info, State03>,
    pub supervisor: Signer<'info>,
    #[account(mut)] pub bucket_a: Account<'info, TokenAccount>,
    #[account(mut)] pub bucket_b: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub external_program: AccountInfo<'info>,               // ← AccountInfo
}
impl<'info> Run03<'info> {
    fn ap(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        CpiContext::new(p, Approve{ to: self.bucket_a.to_account_info(), delegate: self.bucket_b.to_account_info(), authority: self.supervisor.to_account_info() })
    }
    fn tr(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(p, Transfer{ from: self.bucket_a.to_account_info(), to: self.bucket_b.to_account_info(), authority: self.supervisor.to_account_info() })
    }
    fn rv(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        CpiContext::new(p, Revoke{ source: self.bucket_a.to_account_info(), authority: self.supervisor.to_account_info() })
    }
}
#[account] pub struct State03 { pub supervisor: Pubkey, pub step: u64, pub cap: u64, pub total: u64 }
#[error_code] pub enum E03 { #[msg("cap reached")] Cap }
