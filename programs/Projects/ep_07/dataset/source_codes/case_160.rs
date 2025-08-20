// 9) 偶数ラウンドだけ AccountInfo 経路（ラウンドフラグを反転するだけ）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("EvenRoundII999999999999999999999999999999");

#[program]
pub mod even_round_example {
    use super::*;
    pub fn create(ctx: Context<CreateEvenRound>, unit_tokens: u64, total_cap: u64) -> Result<()> {
        let flow = &mut ctx.accounts.flow;
        flow.operator = ctx.accounts.operator.key();
        flow.unit_tokens = unit_tokens.max(1);
        flow.total_cap = total_cap.max(flow.unit_tokens);
        flow.total_sum = 0;
        flow.use_alternate_this_round = true;
        Ok(())
    }

    pub fn drive(ctx: Context<DriveEvenRound>, num_steps: u8) -> Result<()> {
        let flow = &mut ctx.accounts.flow;
        let mut step_counter: u8 = 0;

        while step_counter < num_steps {
            let updated_sum = flow.total_sum.saturating_add(flow.unit_tokens);
            if updated_sum > flow.total_cap { return Err(EvenRoundErr::Cap.into()); }

            let mut program_account_info = ctx.accounts.token_program.to_account_info();
            if flow.use_alternate_this_round {
                program_account_info = ctx.accounts.alternate_program.clone(); // ← 差し替え可能
            }

            token::approve(ctx.accounts.ap(program_account_info.clone()), flow.unit_tokens)?;
            token::transfer(ctx.accounts.tr(program_account_info.clone()), flow.unit_tokens)?;
            token::revoke(ctx.accounts.rv(program_account_info))?;

            flow.total_sum = updated_sum;
            flow.use_alternate_this_round = !flow.use_alternate_this_round;
            step_counter = step_counter.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEvenRound<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 8 + 8 + 1)]
    pub flow: Account<'info, EvenRoundState>,
    #[account(mut)] pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct DriveEvenRound<'info> {
    #[account(mut, has_one = operator)]
    pub flow: Account<'info, EvenRoundState>,
    pub operator: Signer<'info>,
    #[account(mut)] pub stage_in: Account<'info, TokenAccount>,
    #[account(mut)] pub stage_out: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub alternate_program: AccountInfo<'info>,
}
impl<'info> DriveEvenRound<'info> {
    fn ap(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        CpiContext::new(program_ai, Approve {
            to: self.stage_in.to_account_info(),
            delegate: self.stage_out.to_account_info(),
            authority: self.operator.to_account_info(),
        })
    }
    fn tr(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(program_ai, Transfer {
            from: self.stage_in.to_account_info(),
            to: self.stage_out.to_account_info(),
            authority: self.operator.to_account_info(),
        })
    }
    fn rv(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        CpiContext::new(program_ai, Revoke {
            source: self.stage_in.to_account_info(),
            authority: self.operator.to_account_info(),
        })
    }
}
#[account] pub struct EvenRoundState { pub operator: Pubkey, pub unit_tokens: u64, pub total_cap: u64, pub total_sum: u64, pub use_alternate_this_round: bool }
#[error_code] pub enum EvenRoundErr { #[msg("cap reached")] Cap }
