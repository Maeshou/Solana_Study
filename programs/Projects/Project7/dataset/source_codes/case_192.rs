// 8) しきい値を超えた後は外部プログラム、超えるたびしきい値を増やす（moving threshold）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("MovingThreshHH888888888888888888888888888");

#[program]
pub mod moving_threshold_example {
    use super::*;
    pub fn configure(ctx: Context<ConfigureMovingThreshold>, unit_value: u64, cap_value: u64, start_threshold: u64, threshold_increment: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.operator = ctx.accounts.operator.key();
        state.unit_value = unit_value.max(1);
        state.cap_value = cap_value.max(state.unit_value);
        state.current_threshold = start_threshold.max(1);
        state.threshold_increment = threshold_increment.max(1);
        state.total_value = 0;
        Ok(())
    }

    pub fn flow(ctx: Context<FlowMovingThreshold>, step_loops: u8) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let mut loop_counter: u8 = 0;

        while loop_counter < step_loops {
            let next_value = state.total_value.saturating_add(state.unit_value);
            if next_value > state.cap_value { return Err(MtErr::Cap.into()); }

            let mut program_account_info = ctx.accounts.token_program.to_account_info();
            if state.total_value >= state.current_threshold {
                program_account_info = ctx.accounts.external_program.clone(); // ← 差し替え可能
            }

            token::approve(ctx.accounts.a(program_account_info.clone()), state.unit_value)?;
            token::transfer(ctx.accounts.t(program_account_info.clone()), state.unit_value)?;
            token::revoke(ctx.accounts.r(program_account_info))?;

            state.total_value = next_value;
            if state.total_value >= state.current_threshold {
                state.current_threshold = state.current_threshold.saturating_add(state.threshold_increment);
            }
            loop_counter = loop_counter.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConfigureMovingThreshold<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 8 + 8 + 8 + 8)]
    pub state: Account<'info, MovingThresholdState>,
    #[account(mut)] pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct FlowMovingThreshold<'info> {
    #[account(mut, has_one = operator)]
    pub state: Account<'info, MovingThresholdState>,
    pub operator: Signer<'info>,
    #[account(mut)] pub pipe_source: Account<'info, TokenAccount>,
    #[account(mut)] pub pipe_target: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub external_program: AccountInfo<'info>,
}
impl<'info> FlowMovingThreshold<'info> {
    fn a(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        CpiContext::new(program_ai, Approve {
            to: self.pipe_source.to_account_info(),
            delegate: self.pipe_target.to_account_info(),
            authority: self.operator.to_account_info(),
        })
    }
    fn t(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(program_ai, Transfer {
            from: self.pipe_source.to_account_info(),
            to: self.pipe_target.to_account_info(),
            authority: self.operator.to_account_info(),
        })
    }
    fn r(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        CpiContext::new(program_ai, Revoke {
            source: self.pipe_source.to_account_info(),
            authority: self.operator.to_account_info(),
        })
    }
}
#[account] pub struct MovingThresholdState { pub operator: Pubkey, pub unit_value: u64, pub cap_value: u64, pub current_threshold: u64, pub threshold_increment: u64, pub total_value: u64 }
#[error_code] pub enum MtErr { #[msg("cap reached")] Cap }
