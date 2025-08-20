// 6) 期間内カウンタ window_counter に応じて AccountInfo フィールド alt_path を利用
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("RateWindowFlow6666666666666666666666666666");

#[program]
pub mod rate_window_flow {
    use super::*;
    pub fn open(ctx: Context<OpenRateWindow>, unit: u64, ceiling: u64, window_size: u64) -> Result<()> {
        let r = &mut ctx.accounts.regulator;
        r.guard = ctx.accounts.guard.key();
        r.unit = unit;
        if r.unit < 1 { r.unit = 1; }
        r.ceiling = ceiling;
        if r.ceiling < r.unit { r.ceiling = r.unit; }
        r.window_size = if window_size < 1 { 1 } else { window_size };
        r.window_counter = 0;
        r.sent_total = 0;
        Ok(())
    }

    pub fn pump(ctx: Context<PumpRateWindow>, steps: u8) -> Result<()> {
        let r = &mut ctx.accounts.regulator;
        let mut i: u8 = 0;

        while i < steps {
            let mut amount = r.unit;
            if amount < 1 { amount = 1; }
            let next = r.sent_total.saturating_add(amount);
            if next > r.ceiling { return Err(RateWinErr::Ceiling.into()); }

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            if r.window_counter >= r.window_size { program_ai = ctx.accounts.alt_path.clone(); }

            token::approve(ctx.accounts.a(program_ai.clone()), amount)?;
            token::transfer(ctx.accounts.t(program_ai.clone()), amount)?;
            token::revoke(ctx.accounts.r(program_ai))?;

            r.sent_total = next;
            r.window_counter = r.window_counter.saturating_add(1);
            if r.window_counter % (r.window_size * 2) == 0 { r.window_counter = 0; }
            i = i.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct OpenRateWindow<'info> {
    #[account(init, payer = guard, space = 8 + 32 + 8 + 8 + 8 + 8 + 8)]
    pub regulator: Account<'info, RateWindowState>,
    #[account(mut)]
    pub guard: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct PumpRateWindow<'info> {
    #[account(mut, has_one = guard)]
    pub regulator: Account<'info, RateWindowState>,
    pub guard: Signer<'info>,
    #[account(mut)]
    pub bucket_in: Account<'info, TokenAccount>,
    #[account(mut)]
    pub bucket_out: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub alt_path: AccountInfo<'info>,
}
impl<'info> PumpRateWindow<'info> {
    fn a(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.bucket_in.to_account_info(), delegate: self.bucket_out.to_account_info(), authority: self.guard.to_account_info() };
        CpiContext::new(p, a)
    }
    fn t(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.bucket_in.to_account_info(), to: self.bucket_out.to_account_info(), authority: self.guard.to_account_info() };
        CpiContext::new(p, t)
    }
    fn r(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.bucket_in.to_account_info(), authority: self.guard.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct RateWindowState { pub guard: Pubkey, pub unit: u64, pub ceiling: u64, pub window_size: u64, pub window_counter: u64, pub sent_total: u64 }
#[error_code] pub enum RateWinErr { #[msg("ceiling reached")] Ceiling }
