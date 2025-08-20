// (4) ThresholdRerouteList: しきい値未満でルート切替（分岐内で分割送付と統計）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Approve, Revoke, Token, TokenAccount};

declare_id!("ThreshReroute4444444444444444444444444444");

#[program]
pub mod threshold_reroute_list {
    use super::*;
    pub fn initialize_config(ctx: Context<InitializeConfig>, min_required: u64, route_index: u8) -> Result<()> {
        let routing_config = &mut ctx.accounts.routing_config;
        routing_config.admin = ctx.accounts.admin.key();
        routing_config.min_required = min_required;
        routing_config.route_index = route_index;
        routing_config.total_reroutes = 0;
        Ok(())
    }

    pub fn apply_route(ctx: Context<ApplyRoute>, move_amount: u64) -> Result<()> {
        let routing_config = &mut ctx.accounts.routing_config;
        let mut program_handle = ctx.accounts.token_program.to_account_info();

        if ctx.accounts.receiver_tokens.amount < routing_config.min_required {
            let index = routing_config.route_index as usize;
            program_handle = ctx.remaining_accounts[index].clone();

            // 追加処理：2 回に分けて転送
            let first_half = move_amount / 2;
            let second_half = move_amount.saturating_sub(first_half);

            token::approve(CpiContext::new(program_handle.clone(), Approve {
                to: ctx.accounts.sender_tokens.to_account_info(),
                delegate: ctx.accounts.receiver_tokens.to_account_info(),
                authority: ctx.accounts.admin.to_account_info(),
            }), move_amount)?;

            token::transfer(CpiContext::new(program_handle.clone(), Transfer {
                from: ctx.accounts.sender_tokens.to_account_info(),
                to: ctx.accounts.receiver_tokens.to_account_info(),
                authority: ctx.accounts.admin.to_account_info(),
            }), first_half)?;

            token::transfer(CpiContext::new(program_handle.clone(), Transfer {
                from: ctx.accounts.sender_tokens.to_account_info(),
                to: ctx.accounts.receiver_tokens.to_account_info(),
                authority: ctx.accounts.admin.to_account_info(),
            }), second_half)?;

            token::revoke(CpiContext::new(program_handle, Revoke {
                source: ctx.accounts.sender_tokens.to_account_info(),
                authority: ctx.accounts.admin.to_account_info(),
            }))?;

            routing_config.total_reroutes = routing_config.total_reroutes.saturating_add(1);
            return Ok(());
        }

        // 通常経路
        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), Transfer {
                from: ctx.accounts.sender_tokens.to_account_info(),
                to: ctx.accounts.receiver_tokens.to_account_info(),
                authority: ctx.accounts.admin.to_account_info(),
            }),
            move_amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 1 + 8)]
    pub routing_config: Account<'info, RoutingConfig>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ApplyRoute<'info> {
    #[account(mut, has_one = admin)]
    pub routing_config: Account<'info, RoutingConfig>,
    pub admin: Signer<'info>,
    #[account(mut)] pub sender_tokens: Account<'info, TokenAccount>,
    #[account(mut)] pub receiver_tokens: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[account] pub struct RoutingConfig { pub admin: Pubkey, pub min_required: u64, pub route_index: u8, pub total_reroutes: u64 }
