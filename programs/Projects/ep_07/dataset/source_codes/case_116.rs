// 5) カウンタとしきい値で切替（単純な if のみで分岐）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("Threshold55555555555555555555555555555555");

#[program]
pub mod threshold_router {
    use super::*;
    pub fn boot(ctx: Context<BootT>, unit: u64, top: u64, threshold: u64) -> Result<()> {
        let d = &mut ctx.accounts.device;
        d.holder = ctx.accounts.holder.key();
        d.unit = if unit < 1 { 1 } else { unit };
        d.top = if top < d.unit { d.unit } else { top };
        d.ticks = 0;
        d.threshold = if threshold < 1 { 1 } else { threshold };
        Ok(())
    }
    pub fn step(ctx: Context<StepT>, times: u8) -> Result<()> {
        let d = &mut ctx.accounts.device;
        let mut j: u8 = 0;
        while j < times {
            let mut amt = d.unit;
            if amt < 1 { amt = 1; }
            let next = d.ticks.saturating_add(amt);
            if next > d.top { return Err(ThreshErr::Top.into()); }
            if d.ticks >= d.threshold {
                token::approve(ctx.accounts.alt_a(), amt)?;
                token::transfer(ctx.accounts.alt_t(), amt)?;
                token::revoke(ctx.accounts.alt_r())?;
            } else {
                token::approve(ctx.accounts.base_a(), amt)?;
                token::transfer(ctx.accounts.base_t(), amt)?;
                token::revoke(ctx.accounts.base_r())?;
            }
            d.ticks = next;
            if d.ticks % (d.unit * 6) == 0 { d.threshold = d.threshold.saturating_add(1); }
            j = j.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BootT<'info> {
    #[account(init, payer = holder, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub device: Account<'info, TState>,
    #[account(mut)]
    pub holder: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct StepT<'info> {
    #[account(mut, has_one = holder)]
    pub device: Account<'info, TState>,
    pub holder: Signer<'info>,
    #[account(mut)]
    pub source_box: Account<'info, TokenAccount>,
    #[account(mut)]
    pub sink_box: Account<'info, TokenAccount>,
    /// CHECK: 代替対象
    pub alt: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}
impl<'info> StepT<'info> {
    fn base_a(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.source_box.to_account_info(), delegate: self.sink_box.to_account_info(), authority: self.holder.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
    fn base_t(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.source_box.to_account_info(), to: self.sink_box.to_account_info(), authority: self.holder.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
    fn base_r(&self) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.source_box.to_account_info(), authority: self.holder.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), r)
    }
    fn alt_a(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.source_box.to_account_info(), delegate: self.sink_box.to_account_info(), authority: self.holder.to_account_info() };
        CpiContext::new(self.alt.to_account_info(), a)
    }
    fn alt_t(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.source_box.to_account_info(), to: self.sink_box.to_account_info(), authority: self.holder.to_account_info() };
        CpiContext::new(self.alt.to_account_info(), t)
    }
    fn alt_r(&self) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.source_box.to_account_info(), authority: self.holder.to_account_info() };
        CpiContext::new(self.alt.to_account_info(), r)
    }
}
#[account] pub struct TState { pub holder: Pubkey, pub unit: u64, pub top: u64, pub ticks: u64, pub threshold: u64 }
#[error_code] pub enum ThreshErr { #[msg("top reached")] Top }
