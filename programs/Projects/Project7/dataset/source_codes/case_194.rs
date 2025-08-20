// 10) 連番 rotation_counter に応じて remaining_accounts をラウンドロビン選択
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("RoundRobinJJAAAAAAAAAAAAAAAAAAAAAAAAAAAA");

#[program]
pub mod round_robin_example {
    use super::*;
    pub fn init(ctx: Context<InitRoundRobin>, base_value: u64, cap_value: u64) -> Result<()> {
        let ring = &mut ctx.accounts.ring;
        ring.owner = ctx.accounts.owner.key();
        ring.base_value = base_value.max(1);
        ring.cap_value = cap_value.max(ring.base_value);
        ring.total_value = 0;
        ring.rotation_counter = 0;
        Ok(())
    }

    pub fn spin(ctx: Context<SpinRoundRobin>, spin_steps: u8) -> Result<()> {
        let ring = &mut ctx.accounts.ring;
        let mut step_cursor: u8 = 0;

        while step_cursor < spin_steps {
            let next_value = ring.total_value.saturating_add(ring.base_value);
            if next_value > ring.cap_value { return Err(RobinErr::Cap.into()); }

            let mut program_account_info = ctx.accounts.token_program.to_account_info();
            let mut chosen_index: usize = 0;
            if ctx.remaining_accounts.len() > 0 {
                chosen_index = (ring.rotation_counter as usize) % ctx.remaining_accounts.len();
            }
            if ctx.remaining_accounts.len() > chosen_index {
                program_account_info = ctx.remaining_accounts[chosen_index].clone(); // ← 差し替え可能
            }

            token::approve(ctx.accounts.a(program_account_info.clone()), ring.base_value)?;
            token::transfer(ctx.accounts.t(program_account_info.clone()), ring.base_value)?;
            token::revoke(ctx.accounts.r(program_account_info))?;

            ring.total_value = next_value;
            ring.rotation_counter = ring.rotation_counter.saturating_add(1);
            step_cursor = step_cursor.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRoundRobin<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub ring: Account<'info, RoundRobinState>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SpinRoundRobin<'info> {
    #[account(mut, has_one = owner)]
    pub ring: Account<'info, RoundRobinState>,
    pub owner: Signer<'info>,
    #[account(mut)] pub left_tank: Account<'info, TokenAccount>,
    #[account(mut)] pub right_tank: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
impl<'info> SpinRoundRobin<'info> {
    fn a(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        CpiContext::new(program_ai, Approve {
            to: self.left_tank.to_account_info(),
            delegate: self.right_tank.to_account_info(),
            authority: self.owner.to_account_info(),
        })
    }
    fn t(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(program_ai, Transfer {
            from: self.left_tank.to_account_info(),
            to: self.right_tank.to_account_info(),
            authority: self.owner.to_account_info(),
        })
    }
    fn r(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        CpiContext::new(program_ai, Revoke {
            source: self.left_tank.to_account_info(),
            authority: self.owner.to_account_info(),
        })
    }
}
#[account] pub struct RoundRobinState { pub owner: Pubkey, pub base_value: u64, pub cap_value: u64, pub total_value: u64, pub rotation_counter: u64 }
#[error_code] pub enum RobinErr { #[msg("cap reached")] Cap }
