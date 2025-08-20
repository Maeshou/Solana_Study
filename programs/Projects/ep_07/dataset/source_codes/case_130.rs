// 5) approve は Token、transfer/revoke を remaining_accounts[0] に切替（混在パス）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("MixedPhaseFlow2555555555555555555555555555");

#[program]
pub mod mixed_phase_flow2 {
    use super::*;
    pub fn prime(ctx: Context<PrimeMixed2>, base: u64, roof: u64) -> Result<()> {
        let m = &mut ctx.accounts.metrics;
        m.admin = ctx.accounts.admin.key();
        m.base = base;
        if m.base < 1 { m.base = 1; }
        m.roof = roof;
        if m.roof < m.base { m.roof = m.base; }
        m.accum = 0;
        Ok(())
    }

    pub fn push(ctx: Context<PushMixed2>, cycles: u8) -> Result<()> {
        let m = &mut ctx.accounts.metrics;
        let mut c: u8 = 0;
        while c < cycles {
            let mut amount = m.base;
            if amount < 1 { amount = 1; }
            let next = m.accum.saturating_add(amount);
            if next > m.roof { return Err(Mixed2Err::Roof.into()); }

            token::approve(ctx.accounts.approve_token(), amount)?;

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            if ctx.remaining_accounts.len() > 0 {
                program_ai = ctx.remaining_accounts[0].clone();
            }
            token::transfer(ctx.accounts.transfer_with(program_ai.clone()), amount)?;
            token::revoke(ctx.accounts.revoke_with(program_ai))?;

            m.accum = next;
            if m.accum % (m.base * 3) == 0 { m.accum = m.accum; }
            c = c.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PrimeMixed2<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8 + 8)]
    pub metrics: Account<'info, Mixed2State>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct PushMixed2<'info> {
    #[account(mut, has_one = admin)]
    pub metrics: Account<'info, Mixed2State>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub tank_from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub tank_to: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
impl<'info> PushMixed2<'info> {
    fn approve_token(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.tank_from.to_account_info(), delegate: self.tank_to.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
    fn transfer_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.tank_from.to_account_info(), to: self.tank_to.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(p, t)
    }
    fn revoke_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.tank_from.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct Mixed2State { pub admin: Pubkey, pub base: u64, pub roof: u64, pub accum: u64 }
#[error_code] pub enum Mixed2Err { #[msg("roof reached")] Roof }
