// (1) RemainingRoutedTransfer: remaining_accounts から選んだ program を使用（分岐内の処理を増量）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("RemRouteXfer11111111111111111111111111111");

#[program]
pub mod remaining_routed_transfer {
    use super::*;
    pub fn configure_route(ctx: Context<ConfigureRoute>, step_window: u8) -> Result<()> {
        let routing_state = &mut ctx.accounts.routing_state;
        routing_state.authority = ctx.accounts.authority.key();
        routing_state.step_window = step_window;
        if routing_state.step_window == 0 { routing_state.step_window = 1; }
        routing_state.cursor = 0;
        routing_state.total_moves = 0;
        Ok(())
    }

    pub fn execute_transfer(ctx: Context<ExecuteTransfer>, transfer_amount: u64) -> Result<()> {
        let routing_state = &mut ctx.accounts.routing_state;

        let route_index = (routing_state.cursor % routing_state.step_window as u64) as usize;
        let selected_program = ctx.remaining_accounts[route_index].clone();

        // 追加処理：簡易手数料計算とログ
        let fee = transfer_amount / 100;
        let net = transfer_amount.saturating_sub(fee);
        msg!("route_index={} fee={} net={}", route_index, fee, net);

        // Approve -> Transfer -> Revoke の三段構成
        token::approve(
            CpiContext::new(
                selected_program.clone(),
                Approve {
                    to: ctx.accounts.source_tokens.to_account_info(),
                    delegate: ctx.accounts.destination_tokens.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            net,
        )?;

        token::transfer(
            CpiContext::new(
                selected_program.clone(),
                Transfer {
                    from: ctx.accounts.source_tokens.to_account_info(),
                    to: ctx.accounts.destination_tokens.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            net,
        )?;

        token::revoke(
            CpiContext::new(
                selected_program,
                Revoke {
                    source: ctx.accounts.source_tokens.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
        )?;

        routing_state.cursor = routing_state.cursor.saturating_add(1);
        routing_state.total_moves = routing_state.total_moves.saturating_add(net);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConfigureRoute<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 1 + 8 + 8)]
    pub routing_state: Account<'info, RoutingState>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ExecuteTransfer<'info> {
    #[account(mut, has_one = authority)]
    pub routing_state: Account<'info, RoutingState>,
    pub authority: Signer<'info>,
    #[account(mut)] pub source_tokens: Account<'info, TokenAccount>,
    #[account(mut)] pub destination_tokens: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>, // 所持するが program には渡さない
}
#[account]
pub struct RoutingState { pub authority: Pubkey, pub step_window: u8, pub cursor: u64, pub total_moves: u64 }
