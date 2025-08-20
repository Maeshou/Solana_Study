// 4) 実行引数 prefer_external に応じて AccountInfo 経路を選択（パラメトリック）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("ParamChoice444444444444444444444444444444");

#[program]
pub mod param_choice_selector {
    use super::*;
    pub fn configure(ctx: Context<ConfigureParam>, base: u64, cap: u64) -> Result<()> {
        let c = &mut ctx.accounts.config;
        c.supervisor = ctx.accounts.supervisor.key();
        c.base = base;
        if c.base < 1 { c.base = 1; }
        c.cap = cap;
        if c.cap < c.base { c.cap = c.base; }
        c.total = 0;
        Ok(())
    }

    pub fn execute(ctx: Context<ExecuteParam>, iterations: u8, prefer_external: bool) -> Result<()> {
        let c = &mut ctx.accounts.config;
        let mut i: u8 = 0;
        while i < iterations {
            let mut amount = c.base;
            if amount < 1 { amount = 1; }
            let after = c.total.saturating_add(amount);
            if after > c.cap { return Err(ParamErr::Cap.into()); }

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            if prefer_external { program_ai = ctx.accounts.external_program.clone(); }

            token::approve(ctx.accounts.approve_with(program_ai.clone()), amount)?;
            token::transfer(ctx.accounts.transfer_with(program_ai.clone()), amount)?;
            token::revoke(ctx.accounts.revoke_with(program_ai))?;

            c.total = after;
            if c.total % (c.base * 2) == 0 { c.total = c.total; }
            i = i.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConfigureParam<'info> {
    #[account(init, payer = supervisor, space = 8 + 32 + 8 + 8 + 8)]
    pub config: Account<'info, ParamState>,
    #[account(mut)] pub supervisor: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ExecuteParam<'info> {
    #[account(mut, has_one = supervisor)]
    pub config: Account<'info, ParamState>,
    pub supervisor: Signer<'info>,
    #[account(mut)] pub input_cell: Account<'info, TokenAccount>,
    #[account(mut)] pub output_cell: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub external_program: AccountInfo<'info>,
}
impl<'info> ExecuteParam<'info> {
    fn approve_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.input_cell.to_account_info(), delegate: self.output_cell.to_account_info(), authority: self.supervisor.to_account_info() };
        CpiContext::new(p, a)
    }
    fn transfer_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.input_cell.to_account_info(), to: self.output_cell.to_account_info(), authority: self.supervisor.to_account_info() };
        CpiContext::new(p, t)
    }
    fn revoke_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.input_cell.to_account_info(), authority: self.supervisor.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct ParamState { pub supervisor: Pubkey, pub base: u64, pub cap: u64, pub total: u64 }
#[error_code] pub enum ParamErr { #[msg("cap reached")] Cap }
