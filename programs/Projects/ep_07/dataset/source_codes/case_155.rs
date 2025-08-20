// 4) approve は Token 固定、transfer/revoke だけ AccountInfo を採用（混在パス）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("MixedPhasesDD4444444444444444444444444444");

#[program]
pub mod mixed_phase_example {
    use super::*;
    pub fn prime(ctx: Context<PrimeMixedPhase>, base_units: u64, max_sum: u64) -> Result<()> {
        let metrics = &mut ctx.accounts.metrics;
        metrics.controller = ctx.accounts.controller.key();
        metrics.base_units = base_units.max(1);
        metrics.max_sum = max_sum.max(metrics.base_units);
        metrics.accumulator = 0;
        Ok(())
    }

    pub fn push(ctx: Context<PushMixedPhase>, repeat_times: u8) -> Result<()> {
        let metrics = &mut ctx.accounts.metrics;
        let mut loops_done: u8 = 0;

        while loops_done < repeat_times {
            let next_sum = metrics.accumulator.saturating_add(metrics.base_units);
            if next_sum > metrics.max_sum { return Err(MixedPhaseErr::Ceiling.into()); }

            token::approve(ctx.accounts.approve_via_token(), metrics.base_units)?;

            let program_account_info = ctx.accounts.any_program.clone(); // ← 差し替え可能
            token::transfer(ctx.accounts.transfer_via(program_account_info.clone()), metrics.base_units)?;
            token::revoke(ctx.accounts.revoke_via(program_account_info))?;

            metrics.accumulator = next_sum;
            loops_done = loops_done.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PrimeMixedPhase<'info> {
    #[account(init, payer = controller, space = 8 + 32 + 8 + 8 + 8)]
    pub metrics: Account<'info, MixedPhaseState>,
    #[account(mut)] pub controller: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct PushMixedPhase<'info> {
    #[account(mut, has_one = controller)]
    pub metrics: Account<'info, MixedPhaseState>,
    pub controller: Signer<'info>,
    #[account(mut)] pub from_tank: Account<'info, TokenAccount>,
    #[account(mut)] pub to_tank: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub any_program: AccountInfo<'info>,
}
impl<'info> PushMixedPhase<'info> {
    fn approve_via_token(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        CpiContext::new(self.token_program.to_account_info(), Approve {
            to: self.from_tank.to_account_info(),
            delegate: self.to_tank.to_account_info(),
            authority: self.controller.to_account_info(),
        })
    }
    fn transfer_via(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(program_ai, Transfer {
            from: self.from_tank.to_account_info(),
            to: self.to_tank.to_account_info(),
            authority: self.controller.to_account_info(),
        })
    }
    fn revoke_via(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        CpiContext::new(program_ai, Revoke {
            source: self.from_tank.to_account_info(),
            authority: self.controller.to_account_info(),
        })
    }
}
#[account] pub struct MixedPhaseState { pub controller: Pubkey, pub base_units: u64, pub max_sum: u64, pub accumulator: u64 }
#[error_code] pub enum MixedPhaseErr { #[msg("ceiling reached")] Ceiling }
