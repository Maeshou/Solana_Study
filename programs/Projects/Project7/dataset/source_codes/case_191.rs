// 7) 時刻の偶奇に応じて remaining_accounts[0] を採用（なければ Token 経路）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("TimeParityGG7777777777777777777777777777");

#[program]
pub mod time_parity_example {
    use super::*;
    pub fn init(ctx: Context<InitTimeParity>, base_amount: u64, max_total: u64) -> Result<()> {
        let panel = &mut ctx.accounts.panel;
        panel.operator = ctx.accounts.operator.key();
        panel.base_amount = base_amount.max(1);
        panel.max_total = max_total.max(panel.base_amount);
        panel.total_value = 0;
        Ok(())
    }

    pub fn tick(ctx: Context<TickTimeParity>, step_count: u8) -> Result<()> {
        let panel = &mut ctx.accounts.panel;
        let unix_time = Clock::get()?.unix_timestamp;
        let mut step_index: u8 = 0;

        while step_index < step_count {
            let next_value = panel.total_value.saturating_add(panel.base_amount);
            if next_value > panel.max_total { return Err(TimeParityErr::Max.into()); }

            let mut program_account_info = ctx.accounts.token_program.to_account_info();
            let is_odd_second = (unix_time % 2) != 0;
            if is_odd_second {
                if ctx.remaining_accounts.len() > 0 {
                    program_account_info = ctx.remaining_accounts[0].clone(); // ← 差し替え可能
                }
            }

            token::approve(ctx.accounts.approve_with(program_account_info.clone()), panel.base_amount)?;
            token::transfer(ctx.accounts.transfer_with(program_account_info.clone()), panel.base_amount)?;
            token::revoke(ctx.accounts.revoke_with(program_account_info))?;

            panel.total_value = next_value;
            if panel.total_value % (panel.base_amount * 6) == 0 { /* noop */ }
            step_index = step_index.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTimeParity<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 8 + 8)]
    pub panel: Account<'info, TimeParityState>,
    #[account(mut)] pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct TickTimeParity<'info> {
    #[account(mut, has_one = operator)]
    pub panel: Account<'info, TimeParityState>,
    pub operator: Signer<'info>,
    #[account(mut)] pub debit_vault: Account<'info, TokenAccount>,
    #[account(mut)] pub credit_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
impl<'info> TickTimeParity<'info> {
    fn approve_with(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        CpiContext::new(program_ai, Approve {
            to: self.debit_vault.to_account_info(),
            delegate: self.credit_vault.to_account_info(),
            authority: self.operator.to_account_info(),
        })
    }
    fn transfer_with(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(program_ai, Transfer {
            from: self.debit_vault.to_account_info(),
            to: self.credit_vault.to_account_info(),
            authority: self.operator.to_account_info(),
        })
    }
    fn revoke_with(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        CpiContext::new(program_ai, Revoke {
            source: self.debit_vault.to_account_info(),
            authority: self.operator.to_account_info(),
        })
    }
}
#[account] pub struct TimeParityState { pub operator: Pubkey, pub base_amount: u64, pub max_total: u64, pub total_value: u64 }
#[error_code] pub enum TimeParityErr { #[msg("maximum reached")] Max }
