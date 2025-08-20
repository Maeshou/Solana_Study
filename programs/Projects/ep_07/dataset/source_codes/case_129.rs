// 4) 実行パラメータ prefer_external による切り替え（AccountInfo フィールド）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("ParameterChoice444444444444444444444444444");

#[program]
pub mod parameter_choice_flow {
    use super::*;
    pub fn setup(ctx: Context<SetupParamChoice>, step: u64, cap: u64) -> Result<()> {
        let s = &mut ctx.accounts.sched;
        s.owner = ctx.accounts.owner.key();
        s.step = step;
        if s.step < 1 { s.step = 1; }
        s.cap = cap;
        if s.cap < s.step { s.cap = s.step; }
        s.total = 0;
        Ok(())
    }

    pub fn perform(ctx: Context<PerformParamChoice>, rounds: u8, prefer_external: bool) -> Result<()> {
        let s = &mut ctx.accounts.sched;
        let mut i: u8 = 0;

        while i < rounds {
            let mut amount = s.step;
            if amount < 1 { amount = 1; }
            let plan = s.total.saturating_add(amount);
            if plan > s.cap { return Err(ParamChoiceErr::Cap.into()); }

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            if prefer_external { program_ai = ctx.accounts.external_program.clone(); }

            token::approve(ctx.accounts.approve_with(program_ai.clone()), amount)?;
            token::transfer(ctx.accounts.transfer_with(program_ai.clone()), amount)?;
            token::revoke(ctx.accounts.revoke_with(program_ai))?;

            s.total = plan;
            if s.total % (s.step * 2) == 0 { s.total = s.total; }
            i = i.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupParamChoice<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8)]
    pub sched: Account<'info, ParamChoiceState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct PerformParamChoice<'info> {
    #[account(mut, has_one = owner)]
    pub sched: Account<'info, ParamChoiceState>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub vault_in: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault_out: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub external_program: AccountInfo<'info>,
}
impl<'info> PerformParamChoice<'info> {
    fn approve_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.vault_in.to_account_info(), delegate: self.vault_out.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(p, a)
    }
    fn transfer_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.vault_in.to_account_info(), to: self.vault_out.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(p, t)
    }
    fn revoke_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.vault_in.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct ParamChoiceState { pub owner: Pubkey, pub step: u64, pub cap: u64, pub total: u64 }
#[error_code] pub enum ParamChoiceErr { #[msg("capacity reached")] Cap }
