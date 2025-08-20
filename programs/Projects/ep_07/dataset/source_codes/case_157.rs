// 6) ウィンドウ後半のみ AccountInfo へ切替（position >= half_window）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("WindowHalfFF6666666666666666666666666666");

#[program]
pub mod window_half_example {
    use super::*;
    pub fn open(ctx: Context<OpenWindowHalf>, step_units: u64, hard_limit: u64, window_size: u64) -> Result<()> {
        let window = &mut ctx.accounts.window;
        window.owner = ctx.accounts.owner.key();
        window.step_units = step_units.max(1);
        window.hard_limit = hard_limit.max(window.step_units);
        window.window_size = window_size.max(2);
        window.window_position = 0;
        window.total_value = 0;
        Ok(())
    }

    pub fn pump(ctx: Context<PumpWindowHalf>, cycles: u8) -> Result<()> {
        let window = &mut ctx.accounts.window;
        let mut cycle_index: u8 = 0;

        while cycle_index < cycles {
            let next_value = window.total_value.saturating_add(window.step_units);
            if next_value > window.hard_limit { return Err(WindowHalfErr::Cap.into()); }

            let mut program_account_info = ctx.accounts.token_program.to_account_info();
            let half_window = window.window_size / 2;
            if window.window_position >= half_window {
                program_account_info = ctx.accounts.gateway_program.clone(); // ← 差し替え可能
            }

            token::approve(ctx.accounts.a(program_account_info.clone()), window.step_units)?;
            token::transfer(ctx.accounts.t(program_account_info.clone()), window.step_units)?;
            token::revoke(ctx.accounts.r(program_account_info))?;

            window.total_value = next_value;
            window.window_position = window.window_position.saturating_add(1);
            if window.window_position % window.window_size == 0 { window.window_position = 0; }
            cycle_index = cycle_index.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct OpenWindowHalf<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 8 + 8)]
    pub window: Account<'info, WindowHalfState>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct PumpWindowHalf<'info> {
    #[account(mut, has_one = owner)]
    pub window: Account<'info, WindowHalfState>,
    pub owner: Signer<'info>,
    #[account(mut)] pub inlet_box: Account<'info, TokenAccount>,
    #[account(mut)] pub outlet_box: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub gateway_program: AccountInfo<'info>,
}
impl<'info> PumpWindowHalf<'info> {
    fn a(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        CpiContext::new(program_ai, Approve {
            to: self.inlet_box.to_account_info(),
            delegate: self.outlet_box.to_account_info(),
            authority: self.owner.to_account_info(),
        })
    }
    fn t(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(program_ai, Transfer {
            from: self.inlet_box.to_account_info(),
            to: self.outlet_box.to_account_info(),
            authority: self.owner.to_account_info(),
        })
    }
    fn r(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        CpiContext::new(program_ai, Revoke {
            source: self.inlet_box.to_account_info(),
            authority: self.owner.to_account_info(),
        })
    }
}
#[account] pub struct WindowHalfState { pub owner: Pubkey, pub step_units: u64, pub hard_limit: u64, pub window_size: u64, pub window_position: u64, pub total_value: u64 }
#[error_code] pub enum WindowHalfErr { #[msg("cap reached")] Cap }
