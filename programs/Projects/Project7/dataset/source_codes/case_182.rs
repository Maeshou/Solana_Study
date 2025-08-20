// 4) approve は Token、transfer/revoke だけ AccountInfo を使用（混在パス）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("AcctOkArbCpi04DDDDDDDDDDDDDDDDDDDDDDDDDDDD");

#[program]
pub mod acct_ok_arb_cpi_04 {
    use super::*;
    pub fn prime(ctx: Context<Init04>, base: u64, roof: u64) -> Result<()> {
        let m = &mut ctx.accounts.meas;
        m.controller = ctx.accounts.controller.key();
        m.base = base.max(1);
        m.roof = roof.max(m.base);
        m.acc = 0;
        Ok(())
    }
    pub fn push(ctx: Context<Run04>, times: u8) -> Result<()> {
        let m = &mut ctx.accounts.meas;
        let mut n = 0u8;
        while n < times {
            let amt = m.base;
            let next = m.acc.saturating_add(amt);
            if next > m.roof { return Err(E04::Roof.into()); }

            token::approve(ctx.accounts.approve_token(), amt)?; // 固定 Token

            let program_ai = ctx.accounts.any_program.clone();   // ← 差替 AccountInfo
            token::transfer(ctx.accounts.transfer_with(program_ai.clone()), amt)?;
            token::revoke(ctx.accounts.revoke_with(program_ai))?;

            m.acc = next;
            n = n.saturating_add(1);
        }
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Init04<'info> {
    #[account(init, payer = controller, space = 8 + 32 + 8 + 8 + 8)]
    pub meas: Account<'info, State04>,
    #[account(mut)] pub controller: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Run04<'info> {
    #[account(mut, has_one = controller)]
    pub meas: Account<'info, State04>,
    pub controller: Signer<'info>,
    #[account(mut)] pub from_cell: Account<'info, TokenAccount>,
    #[account(mut)] pub to_cell: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub any_program: AccountInfo<'info>,                    // ← AccountInfo
}
impl<'info> Run04<'info> {
    fn approve_token(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        CpiContext::new(self.token_program.to_account_info(),
            Approve{ to: self.from_cell.to_account_info(), delegate: self.to_cell.to_account_info(), authority: self.controller.to_account_info() })
    }
    fn transfer_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(p, Transfer{ from: self.from_cell.to_account_info(), to: self.to_cell.to_account_info(), authority: self.controller.to_account_info() })
    }
    fn revoke_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        CpiContext::new(p, Revoke{ source: self.from_cell.to_account_info(), authority: self.controller.to_account_info() })
    }
}
#[account] pub struct State04 { pub controller: Pubkey, pub base: u64, pub roof: u64, pub acc: u64 }
#[error_code] pub enum E04 { #[msg("roof reached")] Roof }
