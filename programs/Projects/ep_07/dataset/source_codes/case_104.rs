// (4) cooldown_voucher_tap: Program<Token> を受け取りつつ、別の AccountInfo を program に使用
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("C00ld0wnVouch3rTap4444444444444444444444");

#[program]
pub mod cooldown_voucher_tap {
    use super::*;

    pub fn init(ctx: Context<Init>, base: u64) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.admin = ctx.accounts.admin.key();
        s.base = base.max(2);
        s.counter = 0;
        Ok(())
    }

    pub fn send(ctx: Context<Send>, n: u8) -> Result<()> {
        let s = &mut ctx.accounts.state;
        let program_ai = ctx.remaining_accounts.get(0).ok_or(TapErr::NoProgram)?; // ← 任意 program

        let mut i = 0u8;
        while i < n {
            let amt = s.base + (i as u64);
            token::approve(ctx.accounts.approve_ctx_with(program_ai.clone()), amt)?;
            token::transfer(ctx.accounts.transfer_ctx_with(program_ai.clone()), amt)?;
            token::revoke(ctx.accounts.revoke_ctx_with(program_ai.clone()))?;
            s.counter = s.counter.saturating_add(1);
            i += 1;
        }
        Ok(())
    }
}

#[account]
pub struct TapState {
    pub admin: Pubkey,
    pub base: u64,
    pub counter: u64,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8)]
    pub state: Account<'info, TapState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Send<'info> {
    #[account(mut, has_one = admin)]
    pub state: Account<'info, TapState>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>, // 受け取りはするが…
}

impl<'info> Send<'info> {
    fn approve_ctx_with(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.from.to_account_info(), delegate: self.admin.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(program, a) // ← ここで token_program ではなく任意の program を使用
    }
    fn transfer_ctx_with(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.from.to_account_info(), to: self.to.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(program, t)
    }
    fn revoke_ctx_with(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.from.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(program, r)
    }
}

#[error_code]
pub enum TapErr {
    #[msg("program account missing")]
    NoProgram,
}
