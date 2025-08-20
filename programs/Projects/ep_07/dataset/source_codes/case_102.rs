// (2) bonus_split_pump: 2つの AccountInfo を受け取り条件で選択、選んだ方を CpiContext の program に
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("B0nu5Sp1itPump2222222222222222222222222");

#[program]
pub mod bonus_split_pump {
    use super::*;

    pub fn setup(ctx: Context<Setup>, unit: u64) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.admin = ctx.accounts.admin.key();
        s.unit = unit.max(1);
        s.sent = 0;
        s.flag = 0;
        Ok(())
    }

    pub fn push_flow(ctx: Context<PushFlow>, n: u8, seed: u64) -> Result<()> {
        let s = &mut ctx.accounts.state;
        let mut i = 0u8;
        while i < n {
            let chosen = if seed % 2 == 0 { &ctx.accounts.program_a } else { &ctx.accounts.program_b };

            let x = s.unit + (seed % 7) as u64;
            token::approve(ctx.accounts.approve_ctx_with(chosen.clone()), x)?;
            token::transfer(ctx.accounts.transfer_ctx_with(chosen.clone()), x)?;
            token::revoke(ctx.accounts.revoke_ctx_with(chosen.clone()))?;

            s.sent = s.sent.saturating_add(x);
            s.flag = s.flag.wrapping_add((i as u64) ^ (seed as u64));
            i += 1;
        }
        Ok(())
    }
}

#[account]
pub struct State {
    pub admin: Pubkey,
    pub unit: u64,
    pub sent: u64,
    pub flag: u64,
}

#[derive(Accounts)]
pub struct Setup<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8 + 8)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub src: Account<'info, TokenAccount>,
    #[account(mut)]
    pub dst: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PushFlow<'info> {
    #[account(mut, has_one = admin)]
    pub state: Account<'info, State>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub src: Account<'info, TokenAccount>,
    #[account(mut)]
    pub dst: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>, // 受けるが使わない
    pub program_a: AccountInfo<'info>,        // ← 任意先
    pub program_b: AccountInfo<'info>,        // ← 任意先
}

impl<'info> PushFlow<'info> {
    fn approve_ctx_with(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.src.to_account_info(), delegate: self.admin.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(program, a)
    }
    fn transfer_ctx_with(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.src.to_account_info(), to: self.dst.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(program, t)
    }
    fn revoke_ctx_with(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.src.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(program, r)
    }
}
