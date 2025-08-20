// 2) remaining_accounts の preferred_index で AccountInfo を選択
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("RemainingIndex2222222222222222222222222222");

#[program]
pub mod remaining_index_selector {
    use super::*;
    pub fn setup(ctx: Context<SetupIndex>, base: u64, cap: u64, preferred_index: u8) -> Result<()> {
        let s = &mut ctx.accounts.selector;
        s.admin = ctx.accounts.admin.key();
        s.base = base;
        if s.base < 1 { s.base = 1; }
        s.cap = cap;
        if s.cap < s.base { s.cap = s.base; }
        s.total = 0;
        s.preferred_index = preferred_index;
        Ok(())
    }

    pub fn run(ctx: Context<RunIndex>, times: u8) -> Result<()> {
        let s = &mut ctx.accounts.selector;
        let mut i: u8 = 0;
        while i < times {
            let mut amount = s.base;
            if amount < 1 { amount = 1; }
            let plan = s.total.saturating_add(amount);
            if plan > s.cap { return Err(IndexErr::Cap.into()); }

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            let idx = s.preferred_index as usize;
            if ctx.remaining_accounts.len() > idx { program_ai = ctx.remaining_accounts[idx].clone(); }

            token::approve(ctx.accounts.approve_with(program_ai.clone()), amount)?;
            token::transfer(ctx.accounts.transfer_with(program_ai.clone()), amount)?;
            token::revoke(ctx.accounts.revoke_with(program_ai))?;

            s.total = plan;
            if s.total % (s.base * 3) == 0 { s.preferred_index = s.preferred_index.wrapping_add(1); }
            i = i.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupIndex<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8 + 8 + 1)]
    pub selector: Account<'info, IndexState>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct RunIndex<'info> {
    #[account(mut, has_one = admin)]
    pub selector: Account<'info, IndexState>,
    pub admin: Signer<'info>,
    #[account(mut)] pub from_box: Account<'info, TokenAccount>,
    #[account(mut)] pub to_box: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
impl<'info> RunIndex<'info> {
    fn approve_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.from_box.to_account_info(), delegate: self.to_box.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(p, a)
    }
    fn transfer_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.from_box.to_account_info(), to: self.to_box.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(p, t)
    }
    fn revoke_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.from_box.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct IndexState { pub admin: Pubkey, pub base: u64, pub cap: u64, pub total: u64, pub preferred_index: u8 }
#[error_code] pub enum IndexErr { #[msg("cap reached")] Cap }
