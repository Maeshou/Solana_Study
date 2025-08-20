// (10) WindowedSelector: 窓位置で external または remaining_accounts[i] を採用（分岐で二重送付や承認解除）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("WindowSelAAAAABBBBBCCCCCDDDDDEEEEEFFFFF");

#[program]
pub mod windowed_selector {
    use super::*;
    pub fn initialize_selector(ctx: Context<InitializeSelector>, window_size: u64) -> Result<()> {
        let selector_state = &mut ctx.accounts.selector_state;
        selector_state.owner = ctx.accounts.owner.key();
        selector_state.window_size = window_size;
        if selector_state.window_size < 2 { selector_state.window_size = 2; }
        selector_state.position = 0;
        selector_state.double_moves = 0;
        Ok(())
    }

    pub fn tick(ctx: Context<Tick>, transfer_amount: u64) -> Result<()> {
        let selector_state = &mut ctx.accounts.selector_state;
        let window_position = selector_state.position % selector_state.window_size;

        let mut selected_program = ctx.accounts.token_program.to_account_info();

        if window_position == 0 {
            selected_program = ctx.accounts.external_program.clone();

            // 追加：最初の窓だけ二重送付
            token::approve(CpiContext::new(selected_program.clone(), Approve{
                to: ctx.accounts.from_tokens.to_account_info(),
                delegate: ctx.accounts.to_tokens.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            }), transfer_amount)?;

            token::transfer(CpiContext::new(selected_program.clone(), Transfer{
                from: ctx.accounts.from_tokens.to_account_info(),
                to: ctx.accounts.to_tokens.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            }), transfer_amount)?;

            token::transfer(CpiContext::new(selected_program.clone(), Transfer{
                from: ctx.accounts.from_tokens.to_account_info(),
                to: ctx.accounts.to_tokens.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            }), 1)?; // ボーナス 1

            token::revoke(CpiContext::new(selected_program.clone(), Revoke{
                source: ctx.accounts.from_tokens.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            }))?;

            selector_state.double_moves = selector_state.double_moves.saturating_add(1);
        }

        if window_position > 0 {
            let safe_len = ctx.remaining_accounts.len().saturating_sub(1);
            let candidate_index = (window_position as usize - 1).min(safe_len);
            if let Some(candidate_program) = ctx.remaining_accounts.get(candidate_index) {
                selected_program = candidate_program.clone();
            }

            // 通常送付
            token::transfer(CpiContext::new(selected_program.clone(), Transfer{
                from: ctx.accounts.from_tokens.to_account_info(),
                to: ctx.accounts.to_tokens.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            }), transfer_amount)?;
        }

        selector_state.position = selector_state.position.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeSelector<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8)]
    pub selector_state: Account<'info, SelectorState>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Tick<'info> {
    #[account(mut, has_one = owner)]
    pub selector_state: Account<'info, SelectorState>,
    pub owner: Signer<'info>,
    #[account(mut)] pub from_tokens: Account<'info, TokenAccount>,
    #[account(mut)] pub to_tokens: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub external_program: AccountInfo<'info>,
}
#[account] pub struct SelectorState { pub owner: Pubkey, pub window_size: u64, pub position: u64, pub double_moves: u64 }
