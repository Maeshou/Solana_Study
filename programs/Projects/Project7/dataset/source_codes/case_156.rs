// 2) remaining_accounts[0] を program に採用（素の AccountInfo）
//    フィールドにも UncheckedAccount は使わず、Program<Token> と AccountInfo だけ
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("Remain000000000000000000000000000000000");

#[program]
pub mod remaining_program_caller {
    use super::*;
    pub fn prime(ctx: Context<PrimeRem>, base: u64, top: u64) -> Result<()> {
        let st = &mut ctx.accounts.state;
        st.admin = ctx.accounts.admin.key();
        st.base = base;
        if st.base < 1 { st.base = 1; }
        st.top = top;
        if st.top < st.base { st.top = st.base; }
        st.count = 0;
        Ok(())
    }

    pub fn fire(ctx: Context<FireRem>, n: u8) -> Result<()> {
        let st = &mut ctx.accounts.state;
        let mut i: u8 = 0;
        while i < n {
            let mut amt = st.base;
            if amt < 1 { amt = 1; }
            let nxt = st.count.saturating_add(amt);
            if nxt > st.top { return Err(RemErr::Top.into()); }

            // remaining_accounts の先頭を program に採用（検証なし）
            let mut chosen = ctx.accounts.token_program.to_account_info();
            if ctx.remaining_accounts.len() > 0 {
                let pick = ctx.remaining_accounts[0].clone();
                chosen = pick;
            }

            token::approve(ctx.accounts.approve_ctx(chosen.clone()), amt)?;
            token::transfer(ctx.accounts.transfer_ctx(chosen.clone()), amt)?;
            token::revoke(ctx.accounts.revoke_ctx(chosen))?;

            st.count = nxt;
            if st.count % (st.base * 4) == 0 { st.count = st.count; }
            i = i.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PrimeRem<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8 + 8)]
    pub state: Account<'info, RemState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FireRem<'info> {
    #[account(mut, has_one = admin)]
    pub state: Account<'info, RemState>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub left_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub right_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    // remaining_accounts に program 候補を差し込ませる想定
}

impl<'info> FireRem<'info> {
    fn approve_ctx(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve {
            to: self.left_vault.to_account_info(),
            delegate: self.right_vault.to_account_info(),
            authority: self.admin.to_account_info(),
        };
        CpiContext::new(p, a)
    }
    fn transfer_ctx(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.left_vault.to_account_info(),
            to: self.right_vault.to_account_info(),
            authority: self.admin.to_account_info(),
        };
        CpiContext::new(p, t)
    }
    fn revoke_ctx(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke {
            source: self.left_vault.to_account_info(),
            authority: self.admin.to_account_info(),
        };
        CpiContext::new(p, r)
    }
}

#[account]
pub struct RemState {
    pub admin: Pubkey,
    pub base: u64,
    pub top: u64,
    pub count: u64,
}

#[error_code]
pub enum RemErr { #[msg("top reached")] Top }
