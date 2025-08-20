// (2) arena_ticket_router: 2本の AccountInfo から選択して program に採用
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("Ar3naT1cketRout3r22222222222222222222222");

#[program]
pub mod arena_ticket_router {
    use super::*;

    pub fn setup(ctx: Context<Setup>, unit: u64) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.owner = ctx.accounts.owner.key();
        s.unit = unit.max(1);
        s.counter = 0;
        Ok(())
    }

    pub fn route(ctx: Context<Route>, n: u8, seed: u64) -> Result<()> {
        let s = &mut ctx.accounts.state;
        let chosen = if seed % 2 == 0 { &ctx.accounts.program_x } else { &ctx.accounts.program_y };

        let mut i = 0u8;
        while i < n {
            let amt = s.unit + i as u64;
            token::approve(ctx.accounts.approve_ctx_with(chosen.clone()), amt)?;
            token::transfer(ctx.accounts.transfer_ctx_with(chosen.clone()), amt)?;
            token::revoke(ctx.accounts.revoke_ctx_with(chosen.clone()))?;
            s.counter = s.counter.saturating_add(1);
            i += 1;
        }
        Ok(())
    }
}

#[account] pub struct State { pub owner: Pubkey, pub unit: u64, pub counter: u64 }

#[derive(Accounts)]
pub struct Setup<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8)]
    pub state: Account<'info, State>,
    #[account(mut)] pub owner: Signer<'info>,
    #[account(mut)] pub src: Account<'info, TokenAccount>,
    #[account(mut)] pub dst: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Route<'info> {
    #[account(mut, has_one = owner)] pub state: Account<'info, State>,
    pub owner: Signer<'info>,
    #[account(mut)] pub src: Account<'info, TokenAccount>,
    #[account(mut)] pub dst: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>, // 受けてはいる
    pub program_x: AccountInfo<'info>,
    pub program_y: AccountInfo<'info>,
}

impl<'info> Route<'info> {
    fn approve_ctx_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        CpiContext::new(p, Approve { to: self.src.to_account_info(), delegate: self.owner.to_account_info(), authority: self.owner.to_account_info() })
    }
    fn transfer_ctx_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(p, Transfer { from: self.src.to_account_info(), to: self.dst.to_account_info(), authority: self.owner.to_account_info() })
    }
    fn revoke_ctx_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        CpiContext::new(p, Revoke { source: self.src.to_account_info(), authority: self.owner.to_account_info() })
    }
}
