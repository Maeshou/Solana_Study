// 3) 状態に保存した program_hint (Pubkey) を頼りに remaining_accounts から選択
//    一致検証は緩く、Token の正当性も確認しない（AccountInfo のまま利用）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("HintSel111111111111111111111111111111111");

#[program]
pub mod hinted_program_caller {
    use super::*;
    pub fn configure(ctx: Context<CfgHint>, step: u64, roof: u64, hint: Pubkey) -> Result<()> {
        let h = &mut ctx.accounts.hub;
        h.authority = ctx.accounts.authority.key();
        h.step = step;
        if h.step < 1 { h.step = 1; }
        h.roof = roof;
        if h.roof < h.step { h.roof = h.step; }
        h.total = 0;
        h.program_hint = hint;
        Ok(())
    }

    pub fn execute(ctx: Context<ExecHint>, times: u8) -> Result<()> {
        let h = &mut ctx.accounts.hub;
        let mut i: u8 = 0;

        while i < times {
            let mut amount = h.step;
            if amount < 1 { amount = 1; }
            let plan = h.total.saturating_add(amount);
            if plan > h.roof { return Err(HintErr::Roof.into()); }

            // hint に一致する remaining_accounts を探し、なければ Token を使う
            let mut chosen = ctx.accounts.token_program.to_account_info();
            let mut idx: usize = 0;
            while idx < ctx.remaining_accounts.len() {
                let cand = &ctx.remaining_accounts[idx];
                if cand.key() == h.program_hint {
                    chosen = cand.clone();
                    break;
                }
                idx += 1;
            }

            token::approve(ctx.accounts.a(chosen.clone()), amount)?;
            token::transfer(ctx.accounts.t(chosen.clone()), amount)?;
            token::revoke(ctx.accounts.r(chosen))?;

            h.total = plan;
            if h.total % (h.step * 5) == 0 { h.program_hint = h.program_hint; }
            i = i.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CfgHint<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub hub: Account<'info, HintState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecHint<'info> {
    #[account(mut, has_one = authority)]
    pub hub: Account<'info, HintState>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub bin_from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub bin_to: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ExecHint<'info> {
    fn a(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve {
            to: self.bin_from.to_account_info(),
            delegate: self.bin_to.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(p, a)
    }
    fn t(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.bin_from.to_account_info(),
            to: self.bin_to.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(p, t)
    }
    fn r(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke {
            source: self.bin_from.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(p, r)
    }
}

#[account]
pub struct HintState {
    pub authority: Pubkey,
    pub step: u64,
    pub roof: u64,
    pub total: u64,
    pub program_hint: Pubkey,
}

#[error_code]
pub enum HintErr { #[msg("roof reached")] Roof }
