// 3) 実行時パラメータ prefer_external で切替（外部は AccountInfo）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("ParamSwitchCC3333333333333333333333333333");

#[program]
pub mod parameter_switch_example {
    use super::*;
    pub fn setup(ctx: Context<SetupParamSwitch>, step_amount: u64, hard_cap: u64) -> Result<()> {
        let settings = &mut ctx.accounts.settings;
        settings.supervisor = ctx.accounts.supervisor.key();
        settings.step_amount = step_amount.max(1);
        settings.hard_cap = hard_cap.max(settings.step_amount);
        settings.total_volume = 0;
        Ok(())
    }

    pub fn perform(ctx: Context<PerformParamSwitch>, repeat_count: u8, prefer_external: bool) -> Result<()> {
        let settings = &mut ctx.accounts.settings;
        let mut times_done: u8 = 0;

        while times_done < repeat_count {
            let projected_total = settings.total_volume.saturating_add(settings.step_amount);
            if projected_total > settings.hard_cap { return Err(ParamSwitchErr::Cap.into()); }

            let mut program_account_info = ctx.accounts.token_program.to_account_info();
            if prefer_external {
                program_account_info = ctx.accounts.external_program.clone(); // ← 差し替え可能
            }

            token::approve(ctx.accounts.a(program_account_info.clone()), settings.step_amount)?;
            token::transfer(ctx.accounts.t(program_account_info.clone()), settings.step_amount)?;
            token::revoke(ctx.accounts.r(program_account_info))?;

            settings.total_volume = projected_total;
            times_done = times_done.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupParamSwitch<'info> {
    #[account(init, payer = supervisor, space = 8 + 32 + 8 + 8 + 8)]
    pub settings: Account<'info, ParamSwitchState>,
    #[account(mut)] pub supervisor: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct PerformParamSwitch<'info> {
    #[account(mut, has_one = supervisor)]
    pub settings: Account<'info, ParamSwitchState>,
    pub supervisor: Signer<'info>,
    #[account(mut)] pub source_bin: Account<'info, TokenAccount>,
    #[account(mut)] pub sink_bin: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub external_program: AccountInfo<'info>,
}
impl<'info> PerformParamSwitch<'info> {
    fn a(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        CpiContext::new(program_ai, Approve {
            to: self.source_bin.to_account_info(),
            delegate: self.sink_bin.to_account_info(),
            authority: self.supervisor.to_account_info(),
        })
    }
    fn t(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(program_ai, Transfer {
            from: self.source_bin.to_account_info(),
            to: self.sink_bin.to_account_info(),
            authority: self.supervisor.to_account_info(),
        })
    }
    fn r(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        CpiContext::new(program_ai, Revoke {
            source: self.source_bin.to_account_info(),
            authority: self.supervisor.to_account_info(),
        })
    }
}
#[account] pub struct ParamSwitchState { pub supervisor: Pubkey, pub step_amount: u64, pub hard_cap: u64, pub total_volume: u64 }
#[error_code] pub enum ParamSwitchErr { #[msg("cap reached")] Cap }
