// (2) PubkeyMappedProgram: 保存した Pubkey に一致する program を remaining_accounts から検索（分岐処理拡張）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("PkMapProg2222222222222222222222222222222");

#[program]
pub mod pubkey_mapped_program {
    use super::*;
    pub fn set_route(ctx: Context<SetRoute>, target_program_key: Pubkey, daily_cap: u64) -> Result<()> {
        let route_config = &mut ctx.accounts.route_config;
        route_config.admin = ctx.accounts.admin.key();
        route_config.program_key = target_program_key;
        route_config.daily_cap = daily_cap;
        route_config.moved_today = 0;
        route_config.last_day = 0;
        Ok(())
    }

    pub fn hop(ctx: Context<Hop>, move_amount: u64) -> Result<()> {
        let route_config = &mut ctx.accounts.route_config;
        let current_day = (Clock::get()?.unix_timestamp as u64) / 86_400;

        if route_config.last_day != current_day {
            route_config.last_day = current_day;
            route_config.moved_today = 0;
        }
        let planned_total = route_config.moved_today.saturating_add(move_amount);
        require!(planned_total <= route_config.daily_cap, RouteErr::DailyLimit);

        // program を探索
        let wanted_key = route_config.program_key;
        let mut selected_program: Option<AccountInfo> = None;
        for candidate in ctx.remaining_accounts.iter() {
            if candidate.key() == wanted_key { selected_program = Some(candidate.clone()); break; }
        }
        let selected_program = selected_program.ok_or(RouteErr::ProgramNotFound)?;

        // 承認→転送→承認解除
        token::approve(CpiContext::new(
            selected_program.clone(),
            Approve {
                to: ctx.accounts.from_account.to_account_info(),
                delegate: ctx.accounts.to_account.to_account_info(),
                authority: ctx.accounts.admin.to_account_info(),
            }), move_amount)?;

        token::transfer(CpiContext::new(
            selected_program.clone(),
            Transfer {
                from: ctx.accounts.from_account.to_account_info(),
                to: ctx.accounts.to_account.to_account_info(),
                authority: ctx.accounts.admin.to_account_info(),
            }), move_amount)?;

        token::revoke(CpiContext::new(
            selected_program,
            Revoke {
                source: ctx.accounts.from_account.to_account_info(),
                authority: ctx.accounts.admin.to_account_info(),
            }))?;

        route_config.moved_today = planned_total;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetRoute<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 32 + 8 + 8)]
    pub route_config: Account<'info, RouteConfig>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Hop<'info> {
    #[account(mut, has_one = admin)]
    pub route_config: Account<'info, RouteConfig>,
    pub admin: Signer<'info>,
    #[account(mut)] pub from_account: Account<'info, TokenAccount>,
    #[account(mut)] pub to_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[account] pub struct RouteConfig { pub admin: Pubkey, pub program_key: Pubkey, pub daily_cap: u64, pub moved_today: u64, pub last_day: u64 }
#[error_code] pub enum RouteErr { #[msg("program not found")] ProgramNotFound, #[msg("daily limit reached")] DailyLimit }
