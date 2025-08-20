// 1) 管理フラグで外部プログラムへ切替
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("RouteFlagAA111111111111111111111111111111");

#[program]
pub mod route_flag_example {
    use super::*;
    pub fn initialize(ctx: Context<InitializeRouteFlag>, unit_size: u64, max_capacity: u64, use_router_path: bool) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.owner = ctx.accounts.owner.key();
        config.unit_size = unit_size.max(1);
        config.max_capacity = max_capacity.max(config.unit_size);
        config.total_sent = 0;
        config.use_router_path = use_router_path;
        Ok(())
    }

    pub fn process(ctx: Context<ProcessRouteFlag>, num_rounds: u8) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let mut current_round: u8 = 0;

        while current_round < num_rounds {
            let transfer_amount = config.unit_size;
            let next_total = config.total_sent.saturating_add(transfer_amount);
            if next_total > config.max_capacity { return Err(RouteFlagErr::Capacity.into()); }

            let mut program_account_info = ctx.accounts.token_program.to_account_info();
            if config.use_router_path {
                program_account_info = ctx.accounts.router_program.clone(); // ← 差し替え可能
            }

            token::approve(ctx.accounts.approve_ctx(program_account_info.clone()), transfer_amount)?;
            token::transfer(ctx.accounts.transfer_ctx(program_account_info.clone()), transfer_amount)?;
            token::revoke(ctx.accounts.revoke_ctx(program_account_info))?;

            config.total_sent = next_total;
            if config.total_sent % (config.unit_size * 4) == 0 { config.use_router_path = !config.use_router_path; }
            current_round = current_round.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeRouteFlag<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 1)]
    pub config: Account<'info, RouteFlagState>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ProcessRouteFlag<'info> {
    #[account(mut, has_one = owner)]
    pub config: Account<'info, RouteFlagState>,
    pub owner: Signer<'info>,
    #[account(mut)] pub source_vault: Account<'info, TokenAccount>,
    #[account(mut)] pub destination_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub router_program: AccountInfo<'info>,
}
impl<'info> ProcessRouteFlag<'info> {
    fn approve_ctx(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        CpiContext::new(program_ai, Approve {
            to: self.source_vault.to_account_info(),
            delegate: self.destination_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        })
    }
    fn transfer_ctx(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(program_ai, Transfer {
            from: self.source_vault.to_account_info(),
            to: self.destination_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        })
    }
    fn revoke_ctx(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        CpiContext::new(program_ai, Revoke {
            source: self.source_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        })
    }
}
#[account] pub struct RouteFlagState { pub owner: Pubkey, pub unit_size: u64, pub max_capacity: u64, pub total_sent: u64, pub use_router_path: bool }
#[error_code] pub enum RouteFlagErr { #[msg("capacity exceeded")] Capacity }
