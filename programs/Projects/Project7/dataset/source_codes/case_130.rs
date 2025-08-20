// (1) RemainingRoutedTransfer: remaining_accounts から選んだ program を使用
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("RemRouteXfer11111111111111111111111111111");

#[program]
pub mod remaining_routed_transfer {
    use super::*;
    pub fn configure_route(ctx: Context<ConfigureRoute>, step_window: u8) -> Result<()> {
        let routing_state = &mut ctx.accounts.routing_state;
        routing_state.authority = ctx.accounts.authority.key();
        routing_state.step_window = if step_window == 0 { 1 } else { step_window };
        routing_state.cursor = 0;
        Ok(())
    }

    pub fn execute_transfer(ctx: Context<ExecuteTransfer>, transfer_amount: u64) -> Result<()> {
        let routing_state = &mut ctx.accounts.routing_state;
        let route_index = (routing_state.cursor % routing_state.step_window as u64) as usize;
        // program を remaining_accounts から選択
        let selected_program = ctx.remaining_accounts[route_index].clone();

        token::transfer(
            CpiContext::new(
                selected_program,
                Transfer {
                    from: ctx.accounts.source_tokens.to_account_info(),
                    to: ctx.accounts.destination_tokens.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            transfer_amount.max(1),
        )?;
        routing_state.cursor = routing_state.cursor.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConfigureRoute<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 1 + 8)]
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
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct RoutingState { pub authority: Pubkey, pub step_window: u8, pub cursor: u64 }
