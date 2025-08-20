// 8) 二つの閾値で切替（順序付き if を重ねてフラグを決定、AccountInfo 経路あり）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("DualThreshold8888888888888888888888888888");

#[program]
pub mod dual_threshold_selector {
    use super::*;
    pub fn boot(ctx: Context<BootDual>, base: u64, ceiling: u64, threshold_low: u64, threshold_high: u64) -> Result<()> {
        let d = &mut ctx.accounts.dashboard;
        d.owner = ctx.accounts.owner.key();
        d.base = base;
        if d.base < 1 { d.base = 1; }
        d.ceiling = ceiling;
        if d.ceiling < d.base { d.ceiling = d.base; }
        d.threshold_low = if threshold_low < 1 { 1 } else { threshold_low };
        d.threshold_high = if threshold_high < d.threshold_low { d.threshold_low } else { threshold_high };
        d.total = 0;
        Ok(())
    }

    pub fn flow(ctx: Context<FlowDual>, count: u8) -> Result<()> {
        let d = &mut ctx.accounts.dashboard;
        let mut i: u8 = 0;

        while i < count {
            let mut amount = d.base;
            if amount < 1 { amount = 1; }
            let next = d.total.saturating_add(amount);
            if next > d.ceiling { return Err(DualErr::Ceiling.into()); }

            let mut use_external = false;
            if d.total >= d.threshold_low { use_external = true; }
            if d.total >= d.threshold_high { use_external = false; }

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            if use_external { program_ai = ctx.accounts.external_path.clone(); }

            token::approve(ctx.accounts.approve_with(program_ai.clone()), amount)?;
            token::transfer(ctx.accounts.transfer_with(program_ai.clone()), amount)?;
            token::revoke(ctx.accounts.revoke_with(program_ai))?;

            d.total = next;
            if d.total % (d.base * 7) == 0 { d.threshold_low = d.threshold_low.saturating_add(1); }
            i = i.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BootDual<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 8 + 8 + 8)]
    pub dashboard: Account<'info, DualState>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct FlowDual<'info> {
    #[account(mut, has_one = owner)]
    pub dashboard: Account<'info, DualState>,
    pub owner: Signer<'info>,
    #[account(mut)] pub bin_a: Account<'info, TokenAccount>,
    #[account(mut)] pub bin_b: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub external_path: AccountInfo<'info>,
}
impl<'info> FlowDual<'info> {
    fn approve_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.bin_a.to_account_info(), delegate: self.bin_b.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(p, a)
    }
    fn transfer_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.bin_a.to_account_info(), to: self.bin_b.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(p, t)
    }
    fn revoke_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.bin_a.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct DualState { pub owner: Pubkey, pub base: u64, pub ceiling: u64, pub threshold_low: u64, pub threshold_high: u64, pub total: u64 }
#[error_code] pub enum DualErr { #[msg("ceiling reached")] Ceiling }
