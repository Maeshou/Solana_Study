// 5) approve は Token、transfer/revoke は remaining_accounts[0] を採用（混在経路）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("MixedRemain555555555555555555555555555555");

#[program]
pub mod mixed_remaining_transfer {
    use super::*;
    pub fn prepare(ctx: Context<PrepareMixed>, unit: u64, roof: u64) -> Result<()> {
        let m = &mut ctx.accounts.measure;
        m.controller = ctx.accounts.controller.key();
        m.unit = unit;
        if m.unit < 1 { m.unit = 1; }
        m.roof = roof;
        if m.roof < m.unit { m.roof = m.unit; }
        m.accum = 0;
        Ok(())
    }

    pub fn push(ctx: Context<PushMixed>, times: u8) -> Result<()> {
        let m = &mut ctx.accounts.measure;
        let mut n: u8 = 0;
        while n < times {
            let mut amount = m.unit;
            if amount < 1 { amount = 1; }
            let next_val = m.accum.saturating_add(amount);
            if next_val > m.roof { return Err(MixedErr::Roof.into()); }

            token::approve(ctx.accounts.approve_token(), amount)?;

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            if ctx.remaining_accounts.len() > 0 { program_ai = ctx.remaining_accounts[0].clone(); }

            token::transfer(ctx.accounts.transfer_with(program_ai.clone()), amount)?;
            token::revoke(ctx.accounts.revoke_with(program_ai))?;

            m.accum = next_val;
            if m.accum % (m.unit * 3) == 0 { m.accum = m.accum; }
            n = n.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PrepareMixed<'info> {
    #[account(init, payer = controller, space = 8 + 32 + 8 + 8 + 8)]
    pub measure: Account<'info, MixedState>,
    #[account(mut)] pub controller: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct PushMixed<'info> {
    #[account(mut, has_one = controller)]
    pub measure: Account<'info, MixedState>,
    pub controller: Signer<'info>,
    #[account(mut)] pub from_cell: Account<'info, TokenAccount>,
    #[account(mut)] pub to_cell: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
impl<'info> PushMixed<'info> {
    fn approve_token(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.from_cell.to_account_info(), delegate: self.to_cell.to_account_info(), authority: self.controller.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
    fn transfer_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.from_cell.to_account_info(), to: self.to_cell.to_account_info(), authority: self.controller.to_account_info() };
        CpiContext::new(p, t)
    }
    fn revoke_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.from_cell.to_account_info(), authority: self.controller.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct MixedState { pub controller: Pubkey, pub unit: u64, pub roof: u64, pub accum: u64 }
#[error_code] pub enum MixedErr { #[msg("roof reached")] Roof }
