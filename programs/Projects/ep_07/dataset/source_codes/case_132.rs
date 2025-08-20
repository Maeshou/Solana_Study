// 7) epoch_counter を回し、偶数エポックだけ remaining_accounts から program を選択
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("EpochCycleFlow7777777777777777777777777777");

#[program]
pub mod epoch_cycle_flow {
    use super::*;
    pub fn create(ctx: Context<CreateEpoch>, base: u64, peak: u64) -> Result<()> {
        let e = &mut ctx.accounts.epoch;
        e.authority = ctx.accounts.authority.key();
        e.base = base;
        if e.base < 1 { e.base = 1; }
        e.peak = peak;
        if e.peak < e.base { e.peak = e.base; }
        e.used = 0;
        e.epoch_counter = 0;
        Ok(())
    }

    pub fn roll(ctx: Context<RollEpoch>, count: u8) -> Result<()> {
        let e = &mut ctx.accounts.epoch;
        let mut k: u8 = 0;
        while k < count {
            let mut amount = e.base;
            if amount < 1 { amount = 1; }
            let next = e.used.saturating_add(amount);
            if next > e.peak { return Err(EpochErr::Peak.into()); }

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            let even_epoch = (e.epoch_counter % 2) == 0;
            if even_epoch {
                if ctx.remaining_accounts.len() > 0 { program_ai = ctx.remaining_accounts[0].clone(); }
            }

            token::approve(ctx.accounts.ctx_a(program_ai.clone()), amount)?;
            token::transfer(ctx.accounts.ctx_t(program_ai.clone()), amount)?;
            token::revoke(ctx.accounts.ctx_r(program_ai))?;

            e.used = next;
            e.epoch_counter = e.epoch_counter.saturating_add(1);
            k = k.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEpoch<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub epoch: Account<'info, EpochState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct RollEpoch<'info> {
    #[account(mut, has_one = authority)]
    pub epoch: Account<'info, EpochState>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub left_bin: Account<'info, TokenAccount>,
    #[account(mut)]
    pub right_bin: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
impl<'info> RollEpoch<'info> {
    fn ctx_a(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.left_bin.to_account_info(), delegate: self.right_bin.to_account_info(), authority: self.authority.to_account_info() };
        CpiContext::new(p, a)
    }
    fn ctx_t(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.left_bin.to_account_info(), to: self.right_bin.to_account_info(), authority: self.authority.to_account_info() };
        CpiContext::new(p, t)
    }
    fn ctx_r(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.left_bin.to_account_info(), authority: self.authority.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct EpochState { pub authority: Pubkey, pub base: u64, pub peak: u64, pub used: u64, pub epoch_counter: u64 }
#[error_code] pub enum EpochErr { #[msg("peak reached")] Peak }
