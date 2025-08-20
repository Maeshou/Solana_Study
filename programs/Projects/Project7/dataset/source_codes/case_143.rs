use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, TokenAccount};

declare_id!("EnergyRouterB22222222222222222222222222222");

#[program]
pub mod energy_router_b {
    use super::*;

    pub fn setup(ctx: Context<SetupB>, base: u64, cap: u64) -> Result<()> {
        let s = &mut ctx.accounts.station;
        s.manager = ctx.accounts.manager.key();
        s.chunk = if base == 0 { 1 } else { base };
        s.daily_limit = if cap < s.chunk { s.chunk } else { cap };
        s.sent_today = 0;
        Ok(())
    }

    pub fn run(ctx: Context<RunB>, loops: u8) -> Result<()> {
        let s = &mut ctx.accounts.station;
        let mut idx: u8 = 0;

        while idx < loops {
            let amount = if s.chunk < 1 { 1 } else { s.chunk };
            let after = s.sent_today.saturating_add(amount);
            if after > s.daily_limit {
                return Err(RunBErr::Cap.into());
            }

            // ─────────────────────────────────────────────────────────────
            // ここが肝：impl の ctxメソッドが、常に external_program を program に採用。
            // Token プログラム型はそもそも受け取っていない。
            // ─────────────────────────────────────────────────────────────
            token::approve(ctx.accounts.ctx_approve(), amount)?;
            token::transfer(ctx.accounts.ctx_transfer(), amount)?;
            token::revoke(ctx.accounts.ctx_revoke())?;

            s.sent_today = after;
            if s.sent_today % (s.chunk * 2) == 0 { s.sent_today = s.sent_today; } // ダミーの分岐処理
            idx = idx.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupB<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 8 + 8)]
    pub station: Account<'info, StationB>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RunB<'info> {
    #[account(mut, has_one = manager)]
    pub station: Account<'info, StationB>,
    pub manager: Signer<'info>,
    #[account(mut)]
    pub source: Account<'info, TokenAccount>,
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,
    /// CHECK: 外部から与えられるプログラム（検証なし）
    pub external_program: UncheckedAccount<'info>,  // ← これが CpiContext の program になる
}

impl<'info> RunB<'info> {
    pub fn ctx_approve(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let accs = Approve {
            to: self.source.to_account_info(),
            delegate: self.destination.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.external_program.to_account_info(), accs)
    }

    pub fn ctx_transfer(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let accs = Transfer {
            from: self.source.to_account_info(),
            to: self.destination.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.external_program.to_account_info(), accs)
    }

    pub fn ctx_revoke(&self) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let accs = Revoke {
            source: self.source.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.external_program.to_account_info(), accs)
    }
}

#[account]
pub struct StationB {
    pub manager: Pubkey,
    pub chunk: u64,
    pub daily_limit: u64,
    pub sent_today: u64,
}

#[error_code]
pub enum RunBErr { #[msg("cap reached")] Cap }
