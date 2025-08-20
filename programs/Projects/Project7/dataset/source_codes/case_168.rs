// 1) フラグで AccountInfo 経路へ切替（router_program: AccountInfo） 
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("RouteFlagAcctInfo1111111111111111111111111");

#[program]
pub mod route_flag_acctinfo {
    use super::*;
    pub fn initialize(ctx: Context<InitRouteFlag>, unit: u64, cap: u64, use_router: bool) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.owner = ctx.accounts.owner.key();
        state.unit = unit;
        if state.unit < 1 { state.unit = 1; }
        state.cap = cap;
        if state.cap < state.unit { state.cap = state.unit; }
        state.sent = 0;
        state.use_router = use_router;
        Ok(())
    }

    pub fn process(ctx: Context<ProcRouteFlag>, rounds: u8) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let mut loop_counter: u8 = 0;
        while loop_counter < rounds {
            let mut amount = state.unit;
            if amount < 1 { amount = 1; }
            let next_total = state.sent.saturating_add(amount);
            if next_total > state.cap { return Err(RouteFlagErr::Quota.into()); }

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            if state.use_router { program_ai = ctx.accounts.router_program.clone(); }

            token::approve(ctx.accounts.ctx_approve(program_ai.clone()), amount)?;
            token::transfer(ctx.accounts.ctx_transfer(program_ai.clone()), amount)?;
            token::revoke(ctx.accounts.ctx_revoke(program_ai))?;

            state.sent = next_total;
            if state.sent % (state.unit * 4) == 0 { state.use_router = !state.use_router; }
            loop_counter = loop_counter.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRouteFlag<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 1)]
    pub state: Account<'info, RouteFlagState>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ProcRouteFlag<'info> {
    #[account(mut, has_one = owner)]
    pub state: Account<'info, RouteFlagState>,
    pub owner: Signer<'info>,
    #[account(mut)] pub source_vault: Account<'info, TokenAccount>,
    #[account(mut)] pub destination_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub router_program: AccountInfo<'info>,
}
impl<'info> ProcRouteFlag<'info> {
    fn ctx_approve(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.source_vault.to_account_info(), delegate: self.destination_vault.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(p, a)
    }
    fn ctx_transfer(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.source_vault.to_account_info(), to: self.destination_vault.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(p, t)
    }
    fn ctx_revoke(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.source_vault.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct RouteFlagState { pub owner: Pubkey, pub unit: u64, pub cap: u64, pub sent: u64, pub use_router: bool }
#[error_code] pub enum RouteFlagErr { #[msg("quota exceeded")] Quota }
