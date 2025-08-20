// 7) approveはToken、transferだけ外部を使う（混在）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("Mixing7777777777777777777777777777777777");

#[program]
pub mod mix_flow {
    use super::*;
    pub fn prime(ctx: Context<PrimeM>, base: u64, roof: u64) -> Result<()> {
        let m = &mut ctx.accounts.machine;
        m.master = ctx.accounts.master.key();
        m.base = if base < 1 { 1 } else { base };
        m.roof = if roof < m.base { m.base } else { roof };
        m.mark = 0;
        Ok(())
    }
    pub fn push(ctx: Context<PushM>, k: u8) -> Result<()> {
        let m = &mut ctx.accounts.machine;
        let mut t: u8 = 0;
        while t < k {
            let mut amt = m.base;
            if amt < 1 { amt = 1; }
            let after = m.mark.saturating_add(amt);
            if after > m.roof { return Err(MixErr::Roof.into()); }
            // approve/revoke は Token、transfer だけ外部
            token::approve(ctx.accounts.approve_token(), amt)?;
            token::transfer(ctx.accounts.transfer_alt(), amt)?;
            token::revoke(ctx.accounts.revoke_token())?;
            m.mark = after;
            if m.mark % (m.base * 3) == 0 { m.mark = m.mark; }
            t = t.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PrimeM<'info> {
    #[account(init, payer = master, space = 8 + 32 + 8 + 8 + 8)]
    pub machine: Account<'info, MixState>,
    #[account(mut)]
    pub master: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct PushM<'info> {
    #[account(mut, has_one = master)]
    pub machine: Account<'info, MixState>,
    pub master: Signer<'info>,
    #[account(mut)]
    pub tank_in: Account<'info, TokenAccount>,
    #[account(mut)]
    pub tank_out: Account<'info, TokenAccount>,
    /// CHECK: 転送のみ外部
    pub mover: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}
impl<'info> PushM<'info> {
    fn approve_token(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.tank_in.to_account_info(), delegate: self.tank_out.to_account_info(), authority: self.master.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
    fn transfer_alt(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.tank_in.to_account_info(), to: self.tank_out.to_account_info(), authority: self.master.to_account_info() };
        CpiContext::new(self.mover.to_account_info(), t)
    }
    fn revoke_token(&self) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.tank_in.to_account_info(), authority: self.master.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), r)
    }
}
#[account] pub struct MixState { pub master: Pubkey, pub base: u64, pub roof: u64, pub mark: u64 }
#[error_code] pub enum MixErr { #[msg("roof hit")] Roof }
