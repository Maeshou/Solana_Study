// 9) 偶奇カウンタ parity_counter で AccountInfo 経路と Token 経路を交互に使用
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("ParityRouteFlow2999999999999999999999999999");

#[program]
pub mod parity_route_flow2 {
    use super::*;
    pub fn seed(ctx: Context<SeedParity2>, base: u64, maxv: u64) -> Result<()> {
        let s = &mut ctx.accounts.switcher;
        s.admin = ctx.accounts.admin.key();
        s.base = base;
        if s.base < 1 { s.base = 1; }
        s.maxv = maxv;
        if s.maxv < s.base { s.maxv = s.base; }
        s.sent = 0;
        s.parity_counter = 0;
        Ok(())
    }

    pub fn fire(ctx: Context<FireParity2>, times: u8) -> Result<()> {
        let s = &mut ctx.accounts.switcher;
        let mut k: u8 = 0;
        while k < times {
            let mut amount = s.base;
            if amount < 1 { amount = 1; }
            let projected = s.sent.saturating_add(amount);
            if projected > s.maxv { return Err(Parity2Err::Max.into()); }

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            let odd = (s.parity_counter % 2) == 1;
            if odd { program_ai = ctx.accounts.alternate_path.clone(); }

            token::approve(ctx.accounts.approve_with(program_ai.clone()), amount)?;
            token::transfer(ctx.accounts.transfer_with(program_ai.clone()), amount)?;
            token::revoke(ctx.accounts.revoke_with(program_ai))?;

            s.sent = projected;
            s.parity_counter = s.parity_counter.saturating_add(1);
            k = k.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SeedParity2<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub switcher: Account<'info, Parity2State>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct FireParity2<'info> {
    #[account(mut, has_one = admin)]
    pub switcher: Account<'info, Parity2State>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub tank_left: Account<'info, TokenAccount>,
    #[account(mut)]
    pub tank_right: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub alternate_path: AccountInfo<'info>,
}
impl<'info> FireParity2<'info> {
    fn approve_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.tank_left.to_account_info(), delegate: self.tank_right.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(p, a)
    }
    fn transfer_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.tank_left.to_account_info(), to: self.tank_right.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(p, t)
    }
    fn revoke_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.tank_left.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct Parity2State { pub admin: Pubkey, pub base: u64, pub maxv: u64, pub sent: u64, pub parity_counter: u64 }
#[error_code] pub enum Parity2Err { #[msg("maximum reached")] Max }
