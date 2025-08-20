// 1) 状態フラグ use_router に基づき program を切替（AccountInfo フィールド） 
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("RouteFlagFlow11111111111111111111111111111");

#[program]
pub mod route_flag_flow {
    use super::*;
    pub fn init(ctx: Context<InitRouteFlag>, unit: u64, limit: u64, use_router: bool) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.owner = ctx.accounts.owner.key();
        config.unit = unit;
        if config.unit < 1 { config.unit = 1; }
        config.limit = limit;
        if config.limit < config.unit { config.limit = config.unit; }
        config.total_sent = 0;
        config.use_router = use_router;
        Ok(())
    }

    pub fn run(ctx: Context<RunRouteFlag>, iterations: u8) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let mut step: u8 = 0;
        while step < iterations {
            let mut amount = config.unit;
            if amount < 1 { amount = 1; }
            let projected = config.total_sent.saturating_add(amount);
            if projected > config.limit { return Err(RouteFlagErr::Quota.into()); }

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            if config.use_router { program_ai = ctx.accounts.router_program.clone(); }

            token::approve(ctx.accounts.ctx_approve(program_ai.clone()), amount)?;
            token::transfer(ctx.accounts.ctx_transfer(program_ai.clone()), amount)?;
            token::revoke(ctx.accounts.ctx_revoke(program_ai))?;

            config.total_sent = projected;
            if config.total_sent % (config.unit * 4) == 0 { config.use_router = !config.use_router; }
            step = step.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRouteFlag<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 1)]
    pub config: Account<'info, RouteFlagState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct RunRouteFlag<'info> {
    #[account(mut, has_one = owner)]
    pub config: Account<'info, RouteFlagState>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub source_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub destination_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub router_program: AccountInfo<'info>, // ← UncheckedAccount ではなく AccountInfo
}
impl<'info> RunRouteFlag<'info> {
    fn ctx_approve(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let accs = Approve { to: self.source_vault.to_account_info(), delegate: self.destination_vault.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(p, accs)
    }
    fn ctx_transfer(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let accs = Transfer { from: self.source_vault.to_account_info(), to: self.destination_vault.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(p, accs)
    }
    fn ctx_revoke(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let accs = Revoke { source: self.source_vault.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(p, accs)
    }
}
#[account] pub struct RouteFlagState { pub owner: Pubkey, pub unit: u64, pub limit: u64, pub total_sent: u64, pub use_router: bool }
#[error_code] pub enum RouteFlagErr { #[msg("quota exceeded")] Quota }
