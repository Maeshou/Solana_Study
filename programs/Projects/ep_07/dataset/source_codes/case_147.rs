// 2) remaining_accounts のインデックス参照で program を選択（フォールバックは Token）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("AcctOkArbCpi02BBBBBBBBBBBBBBBBBBBBBBBBBBBB");

#[program]
pub mod acct_ok_arb_cpi_02 {
    use super::*;
    pub fn configure(ctx: Context<Cfg02>, base: u64, limit: u64, slot: u8) -> Result<()> {
        let c = &mut ctx.accounts.cfg;
        c.owner = ctx.accounts.owner.key();
        c.base = base.max(1);
        c.limit = limit.max(c.base);
        c.idx = slot;
        c.total = 0;
        Ok(())
    }
    pub fn execute(ctx: Context<Exe02>, reps: u8) -> Result<()> {
        let c = &mut ctx.accounts.cfg;
        let mut k = 0u8;
        while k < reps {
            let amt = c.base;
            let next = c.total.saturating_add(amt);
            if next > c.limit { return Err(E02::Limit.into()); }

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            let want = c.idx as usize;
            if ctx.remaining_accounts.len() > want {
                program_ai = ctx.remaining_accounts[want].clone(); // ← 差替ポイント
            }

            token::approve(ctx.accounts.a(program_ai.clone()), amt)?;
            token::transfer(ctx.accounts.t(program_ai.clone()), amt)?;
            token::revoke(ctx.accounts.r(program_ai))?;

            c.total = next;
            if c.total % (c.base * 4) == 0 { c.idx = c.idx.wrapping_add(1); }
            k = k.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Cfg02<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 1 + 8)]
    pub cfg: Account<'info, State02>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Exe02<'info> {
    #[account(mut, has_one = owner)]
    pub cfg: Account<'info, State02>,
    pub owner: Signer<'info>,
    #[account(mut)] pub from_vault: Account<'info, TokenAccount>,
    #[account(mut)] pub to_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
impl<'info> Exe02<'info> {
    fn a(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> { 
        CpiContext::new(p, Approve{ to: self.from_vault.to_account_info(), delegate: self.to_vault.to_account_info(), authority: self.owner.to_account_info() })
    }
    fn t(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> { 
        CpiContext::new(p, Transfer{ from: self.from_vault.to_account_info(), to: self.to_vault.to_account_info(), authority: self.owner.to_account_info() })
    }
    fn r(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> { 
        CpiContext::new(p, Revoke{ source: self.from_vault.to_account_info(), authority: self.owner.to_account_info() })
    }
}
#[account] pub struct State02 { pub owner: Pubkey, pub base: u64, pub limit: u64, pub idx: u8, pub total: u64 }
#[error_code] pub enum E02 { #[msg("limit reached")] Limit }
