// 2) remaining_accounts の preferred_index で動的選択
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("RemainIndexBB2222222222222222222222222222");

#[program]
pub mod remaining_index_example {
    use super::*;
    pub fn configure(ctx: Context<ConfigureIndex>, base_amount: u64, capacity_limit: u64, preferred_index: u8) -> Result<()> {
        let selector = &mut ctx.accounts.selector;
        selector.admin = ctx.accounts.admin.key();
        selector.base_amount = base_amount.max(1);
        selector.capacity_limit = capacity_limit.max(selector.base_amount);
        selector.total_processed = 0;
        selector.preferred_index = preferred_index;
        Ok(())
    }

    pub fn execute(ctx: Context<ExecuteIndex>, loop_count: u8) -> Result<()> {
        let selector = &mut ctx.accounts.selector;
        let mut executed_loops: u8 = 0;

        while executed_loops < loop_count {
            let next_total = selector.total_processed.saturating_add(selector.base_amount);
            if next_total > selector.capacity_limit { return Err(IdxErr::Limit.into()); }

            let mut program_account_info = ctx.accounts.token_program.to_account_info();
            let desired = selector.preferred_index as usize;
            if ctx.remaining_accounts.len() > desired {
                program_account_info = ctx.remaining_accounts[desired].clone(); // ← 差し替え可能
            }

            token::approve(ctx.accounts.ctx_approve(program_account_info.clone()), selector.base_amount)?;
            token::transfer(ctx.accounts.ctx_transfer(program_account_info.clone()), selector.base_amount)?;
            token::revoke(ctx.accounts.ctx_revoke(program_account_info))?;

            selector.total_processed = next_total;
            if selector.total_processed % (selector.base_amount * 3) == 0 { selector.preferred_index = selector.preferred_index.wrapping_add(1); }
            executed_loops = executed_loops.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConfigureIndex<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8 + 8 + 1)]
    pub selector: Account<'info, IndexSelectorState>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ExecuteIndex<'info> {
    #[account(mut, has_one = admin)]
    pub selector: Account<'info, IndexSelectorState>,
    pub admin: Signer<'info>,
    #[account(mut)] pub input_vault: Account<'info, TokenAccount>,
    #[account(mut)] pub output_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
impl<'info> ExecuteIndex<'info> {
    fn ctx_approve(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        CpiContext::new(program_ai, Approve {
            to: self.input_vault.to_account_info(),
            delegate: self.output_vault.to_account_info(),
            authority: self.admin.to_account_info(),
        })
    }
    fn ctx_transfer(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(program_ai, Transfer {
            from: self.input_vault.to_account_info(),
            to: self.output_vault.to_account_info(),
            authority: self.admin.to_account_info(),
        })
    }
    fn ctx_revoke(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        CpiContext::new(program_ai, Revoke {
            source: self.input_vault.to_account_info(),
            authority: self.admin.to_account_info(),
        })
    }
}
#[account] pub struct IndexSelectorState { pub admin: Pubkey, pub base_amount: u64, pub capacity_limit: u64, pub total_processed: u64, pub preferred_index: u8 }
#[error_code] pub enum IdxErr { #[msg("limit reached")] Limit }
