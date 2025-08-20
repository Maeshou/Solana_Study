// 1) 明示フィールドとして raw AccountInfo を受け取り、CpiContext の program に採用
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("RawProg111111111111111111111111111111111");

#[program]
pub mod raw_program_caller {
    use super::*;
    pub fn setup(ctx: Context<SetupRaw>, unit: u64, cap: u64) -> Result<()> {
        let s = &mut ctx.accounts.cfg;
        s.owner = ctx.accounts.owner.key();
        s.unit = unit;
        if s.unit < 1 { s.unit = 1; }
        s.cap = cap;
        if s.cap < s.unit { s.cap = s.unit; }
        s.progress = 0;
        s.flip = false;
        Ok(())
    }

    pub fn run(ctx: Context<RunRaw>, rounds: u8) -> Result<()> {
        let s = &mut ctx.accounts.cfg;
        let mut k: u8 = 0;
        while k < rounds {
            let mut amount = s.unit;
            if amount < 1 { amount = 1; }
            let after = s.progress.saturating_add(amount);
            if after > s.cap { return Err(RawErr::Cap.into()); }

            // flip が true のときだけ raw AccountInfo を program に採用
            let program_ai = if s.flip {
                ctx.accounts.dynamic_program.clone()         // ← AccountInfo<'info>
            } else {
                ctx.accounts.token_program.to_account_info()  // ← Program<'info, Token>
            };

            token::approve(ctx.accounts.ctx_approve(program_ai.clone()), amount)?;
            token::transfer(ctx.accounts.ctx_transfer(program_ai.clone()), amount)?;
            token::revoke(ctx.accounts.ctx_revoke(program_ai))?;

            s.progress = after;
            if s.progress % (s.unit * 3) == 0 { s.flip = !s.flip; }
            k = k.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupRaw<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 1)]
    pub cfg: Account<'info, RawState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RunRaw<'info> {
    #[account(mut, has_one = owner)]
    pub cfg: Account<'info, RawState>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub source_box: Account<'info, TokenAccount>,
    #[account(mut)]
    pub sink_box: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    // ← UncheckedAccount ではなく素の AccountInfo。検証がかからない。
    pub dynamic_program: AccountInfo<'info>,
}

impl<'info> RunRaw<'info> {
    fn ctx_approve(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve {
            to: self.source_box.to_account_info(),
            delegate: self.sink_box.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(p, a)
    }
    fn ctx_transfer(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.source_box.to_account_info(),
            to: self.sink_box.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(p, t)
    }
    fn ctx_revoke(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke {
            source: self.source_box.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(p, r)
    }
}

#[account]
pub struct RawState {
    pub owner: Pubkey,
    pub unit: u64,
    pub cap: u64,
    pub progress: u64,
    pub flip: bool,
}

#[error_code]
pub enum RawErr { #[msg("cap reached")] Cap }
