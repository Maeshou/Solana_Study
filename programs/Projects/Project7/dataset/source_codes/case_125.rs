// (8) MidBalanceSwitch: 中間口座の残高に応じて program を切替
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("MidBalSwitch8888888888888888888888888888");

#[program]
pub mod mid_balance_switch {
    use super::*;
    pub fn initialize_threshold(ctx: Context<InitializeThreshold>, minimum_balance: u64) -> Result<()> {
        let threshold_state = &mut ctx.accounts.threshold_state;
        threshold_state.owner = ctx.accounts.owner.key();
        threshold_state.minimum_balance = minimum_balance;
        Ok(())
    }

    pub fn move_funds(ctx: Context<MoveFunds>, transfer_amount: u64) -> Result<()> {
        let mut selected_program = ctx.accounts.token_program.to_account_info();
        if ctx.accounts.intermediate_tokens.amount < ctx.accounts.threshold_state.minimum_balance {
            selected_program = ctx.accounts.external_program.clone();
        }
        token::transfer(CpiContext::new(selected_program, Transfer{
            from: ctx.accounts.source_tokens.to_account_info(),
            to: ctx.accounts.intermediate_tokens.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        }), transfer_amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeThreshold<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8)]
    pub threshold_state: Account<'info, ThresholdState>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MoveFunds<'info> {
    pub threshold_state: Account<'info, ThresholdState>,
    pub owner: Signer<'info>,
    #[account(mut)] pub source_tokens: Account<'info, TokenAccount>,
    #[account(mut)] pub intermediate_tokens: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub external_program: AccountInfo<'info>,
}

#[account] pub struct ThresholdState { pub owner: Pubkey, pub minimum_balance: u64 }
