// 6) 回転インデックス rotation_counter に応じて remaining_accounts から選択（リング風）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("RotationRing666666666666666666666666666666");

#[program]
pub mod rotation_ring_selector {
    use super::*;
    pub fn init(ctx: Context<InitRing>, base: u64, maxv: u64) -> Result<()> {
        let r = &mut ctx.accounts.ring;
        r.owner = ctx.accounts.owner.key();
        r.base = base;
        if r.base < 1 { r.base = 1; }
        r.maxv = maxv;
        if r.maxv < r.base { r.maxv = r.base; }
        r.total = 0;
        r.rotation_counter = 0;
        Ok(())
    }

    pub fn spin(ctx: Context<SpinRing>, steps: u8) -> Result<()> {
        let r = &mut ctx.accounts.ring;
        let mut s: u8 = 0;
        while s < steps {
            let mut amount = r.base;
            if amount < 1 { amount = 1; }
            let next = r.total.saturating_add(amount);
            if next > r.maxv { return Err(RingErr::Max.into()); }

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            let mut pick_index: usize = 0;
            if ctx.remaining_accounts.len() > 0 { pick_index = (r.rotation_counter as usize) % ctx.remaining_accounts.len(); }
            if ctx.remaining_accounts.len() > pick_index { program_ai = ctx.remaining_accounts[pick_index].clone(); }

            token::approve(ctx.accounts.a(program_ai.clone()), amount)?;
            token::transfer(ctx.accounts.t(program_ai.clone()), amount)?;
            token::revoke(ctx.accounts.r(program_ai))?;

            r.total = next;
            r.rotation_counter = r.rotation_counter.saturating_add(1);
            s = s.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRing<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub ring: Account<'info, RingState>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SpinRing<'info> {
    #[account(mut, has_one = owner)]
    pub ring: Account<'info, RingState>,
    pub owner: Signer<'info>,
    #[account(mut)] pub left_tank: Account<'info, TokenAccount>,
    #[account(mut)] pub right_tank: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
impl<'info> SpinRing<'info> {
    fn a(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.left_tank.to_account_info(), delegate: self.right_tank.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(p, a)
    }
    fn t(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.left_tank.to_account_info(), to: self.right_tank.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(p, t)
    }
    fn r(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.left_tank.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct RingState { pub owner: Pubkey, pub base: u64, pub maxv: u64, pub total: u64, pub rotation_counter: u64 }
#[error_code] pub enum RingErr { #[msg("maximum reached")] Max }
