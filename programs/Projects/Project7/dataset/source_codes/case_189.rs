// 5) hint_program_key に一致する remaining_accounts を優先採用（緩い突合せ）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("HintLookupEE5555555555555555555555555555");

#[program]
pub mod hint_lookup_example {
    use super::*;
    pub fn register(ctx: Context<RegisterHint>, unit_amount: u64, cap_total: u64, hint_program_key: Pubkey) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        registry.manager = ctx.accounts.manager.key();
        registry.unit_amount = unit_amount.max(1);
        registry.cap_total = cap_total.max(registry.unit_amount);
        registry.total_processed = 0;
        registry.hint_program_key = hint_program_key;
        Ok(())
    }

    pub fn apply(ctx: Context<ApplyHint>, iterations: u8) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        let mut iteration_index: u8 = 0;

        while iteration_index < iterations {
            let next_total = registry.total_processed.saturating_add(registry.unit_amount);
            if next_total > registry.cap_total { return Err(HintErr::Cap.into()); }

            let mut program_account_info = ctx.accounts.token_program.to_account_info();
            let mut scan_index: usize = 0;
            while scan_index < ctx.remaining_accounts.len() {
                let candidate = &ctx.remaining_accounts[scan_index];
                if candidate.key() == registry.hint_program_key {
                    program_account_info = candidate.clone();      // ← 差し替え可能
                    break;
                }
                scan_index = scan_index.saturating_add(1);
            }

            token::approve(ctx.accounts.approve_with(program_account_info.clone()), registry.unit_amount)?;
            token::transfer(ctx.accounts.transfer_with(program_account_info.clone()), registry.unit_amount)?;
            token::revoke(ctx.accounts.revoke_with(program_account_info))?;

            registry.total_processed = next_total;
            iteration_index = iteration_index.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterHint<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub registry: Account<'info, HintRegistryState>,
    #[account(mut)] pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ApplyHint<'info> {
    #[account(mut, has_one = manager)]
    pub registry: Account<'info, HintRegistryState>,
    pub manager: Signer<'info>,
    #[account(mut)] pub left_store: Account<'info, TokenAccount>,
    #[account(mut)] pub right_store: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
impl<'info> ApplyHint<'info> {
    fn approve_with(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        CpiContext::new(program_ai, Approve {
            to: self.left_store.to_account_info(),
            delegate: self.right_store.to_account_info(),
            authority: self.manager.to_account_info(),
        })
    }
    fn transfer_with(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(program_ai, Transfer {
            from: self.left_store.to_account_info(),
            to: self.right_store.to_account_info(),
            authority: self.manager.to_account_info(),
        })
    }
    fn revoke_with(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        CpiContext::new(program_ai, Revoke {
            source: self.left_store.to_account_info(),
            authority: self.manager.to_account_info(),
        })
    }
}
#[account] pub struct HintRegistryState { pub manager: Pubkey, pub unit_amount: u64, pub cap_total: u64, pub total_processed: u64, pub hint_program_key: Pubkey }
#[error_code] pub enum HintErr { #[msg("cap reached")] Cap }
