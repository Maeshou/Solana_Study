// 10) divider 値により remaining_accounts のインデックスを切替（単純 if と加算のみ）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("DividerThresholdFlowAAAAAAAAAAAAAAAAAAAAAAA");

#[program]
pub mod divider_threshold_flow {
    use super::*;
    pub fn prepare(ctx: Context<PrepareDivider>, unit: u64, top: u64, divider: u64) -> Result<()> {
        let d = &mut ctx.accounts.dashboard;
        d.owner = ctx.accounts.owner.key();
        d.unit = unit;
        if d.unit < 1 { d.unit = 1; }
        d.top = top;
        if d.top < d.unit { d.top = d.unit; }
        d.divider = if divider < 1 { 1 } else { divider };
        d.total = 0;
        Ok(())
    }

    pub fn process(ctx: Context<ProcessDivider>, steps: u8) -> Result<()> {
        let d = &mut ctx.accounts.dashboard;
        let mut s: u8 = 0;
        while s < steps {
            let mut amount = d.unit;
            if amount < 1 { amount = 1; }
            let next = d.total.saturating_add(amount);
            if next > d.top { return Err(DividerErr::Top.into()); }

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            let mut index: usize = 0;
            if d.total >= d.divider { index = 1; }
            if ctx.remaining_accounts.len() > index { program_ai = ctx.remaining_accounts[index].clone(); }

            token::approve(ctx.accounts.approve_ctx(program_ai.clone()), amount)?;
            token::transfer(ctx.accounts.transfer_ctx(program_ai.clone()), amount)?;
            token::revoke(ctx.accounts.revoke_ctx(program_ai))?;

            d.total = next;
            if d.total % (d.unit * 5) == 0 { d.divider = d.divider.saturating_add(1); }
            s = s.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PrepareDivider<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub dashboard: Account<'info, DividerState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ProcessDivider<'info> {
    #[account(mut, has_one = owner)]
    pub dashboard: Account<'info, DividerState>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub tray_from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub tray_to: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
impl<'info> ProcessDivider<'info> {
    fn approve_ctx(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.tray_from.to_account_info(), delegate: self.tray_to.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(p, a)
    }
    fn transfer_ctx(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.tray_from.to_account_info(), to: self.tray_to.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(p, t)
    }
    fn revoke_ctx(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.tray_from.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct DividerState { pub owner: Pubkey, pub unit: u64, pub top: u64, pub divider: u64, pub total: u64 }
#[error_code] pub enum DividerErr { #[msg("top reached")] Top }
