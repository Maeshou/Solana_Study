// 3) 現在時刻の偶奇で AccountInfo 経路を採用（fallback は Token）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("TimeParity3333333333333333333333333333333");

#[program]
pub mod time_parity_selector {
    use super::*;
    pub fn set(ctx: Context<SetParity>, unit: u64, limit: u64) -> Result<()> {
        let p = &mut ctx.accounts.panel;
        p.operator = ctx.accounts.operator.key();
        p.unit = unit;
        if p.unit < 1 { p.unit = 1; }
        p.limit = limit;
        if p.limit < p.unit { p.limit = p.unit; }
        p.progress = 0;
        Ok(())
    }

    pub fn step(ctx: Context<StepParity>, cycles: u8) -> Result<()> {
        let p = &mut ctx.accounts.panel;
        let now = Clock::get()?.unix_timestamp;
        let mut c: u8 = 0;
        while c < cycles {
            let mut amount = p.unit;
            if amount < 1 { amount = 1; }
            let after = p.progress.saturating_add(amount);
            if after > p.limit { return Err(ParityErr::Limit.into()); }

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            let odd = (now % 2) != 0;
            if odd { program_ai = ctx.accounts.alternate_path.clone(); }

            token::approve(ctx.accounts.a(program_ai.clone()), amount)?;
            token::transfer(ctx.accounts.t(program_ai.clone()), amount)?;
            token::revoke(ctx.accounts.r(program_ai))?;

            p.progress = after;
            if p.progress % (p.unit * 6) == 0 { p.progress = p.progress; }
            c = c.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetParity<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 8 + 8)]
    pub panel: Account<'info, ParityState>,
    #[account(mut)] pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct StepParity<'info> {
    #[account(mut, has_one = operator)]
    pub panel: Account<'info, ParityState>,
    pub operator: Signer<'info>,
    #[account(mut)] pub debit: Account<'info, TokenAccount>,
    #[account(mut)] pub credit: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub alternate_path: AccountInfo<'info>,
}
impl<'info> StepParity<'info> {
    fn a(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.debit.to_account_info(), delegate: self.credit.to_account_info(), authority: self.operator.to_account_info() };
        CpiContext::new(p, a)
    }
    fn t(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.debit.to_account_info(), to: self.credit.to_account_info(), authority: self.operator.to_account_info() };
        CpiContext::new(p, t)
    }
    fn r(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.debit.to_account_info(), authority: self.operator.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct ParityState { pub operator: Pubkey, pub unit: u64, pub limit: u64, pub progress: u64 }
#[error_code] pub enum ParityErr { #[msg("limit reached")] Limit }
