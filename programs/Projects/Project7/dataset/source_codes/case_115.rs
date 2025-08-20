// (8) MidBalanceSwitch: 中間口座の残高で切替（分岐内で追加入金とログ）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Approve, Revoke, Token, TokenAccount};

declare_id!("MidBalSwitch8888888888888888888888888888");

#[program]
pub mod mid_balance_switch {
    use super::*;
    pub fn initialize_threshold(ctx: Context<InitializeThreshold>, minimum_balance: u64) -> Result<()> {
        let threshold_state = &mut ctx.accounts.threshold_state;
        threshold_state.owner = ctx.accounts.owner.key();
        threshold_state.minimum_balance = minimum_balance;
        threshold_state.topped_up = 0;
        Ok(())
    }

    pub fn move_funds(ctx: Context<MoveFunds>, transfer_amount: u64) -> Result<()> {
        let mut selected_program = ctx.accounts.token_program.to_account_info();

        if ctx.accounts.intermediate_tokens.amount < ctx.accounts.threshold_state.minimum_balance {
            selected_program = ctx.accounts.external_program.clone();

            // 追加：少額チャージを先に実施
            let top_up = transfer_amount / 10;
            if top_up > 0 {
                token::transfer(CpiContext::new(
                    selected_program.clone(),
                    Transfer {
                        from: ctx.accounts.source_tokens.to_account_info(),
                        to: ctx.accounts.intermediate_tokens.to_account_info(),
                        authority: ctx.accounts.owner.to_account_info(),
                    }), top_up)?;
                ctx.accounts.threshold_state.topped_up = ctx.accounts.threshold_state.topped_up.saturating_add(top_up);
                msg!("intermediate topped up by {}", top_up);
            }
        }

        // 本転送
        token::approve(CpiContext::new(selected_program.clone(), Approve {
            to: ctx.accounts.source_tokens.to_account_info(),
            delegate: ctx.accounts.intermediate_tokens.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        }), transfer_amount)?;

        token::transfer(CpiContext::new(selected_program.clone(), Transfer{
            from: ctx.accounts.source_tokens.to_account_info(),
            to: ctx.accounts.intermediate_tokens.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        }), transfer_amount)?;

        token::revoke(CpiContext::new(selected_program, Revoke{
            source: ctx.accounts.source_tokens.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        }))?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeThreshold<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8)]
    pub threshold_state: Account<'info, ThresholdState>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct MoveFunds<'info> {
    #[account(mut, has_one = owner)]
    pub threshold_state: Account<'info, ThresholdState>,
    pub owner: Signer<'info>,
    #[account(mut)] pub source_tokens: Account<'info, TokenAccount>,
    #[account(mut)] pub intermediate_tokens: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub external_program: AccountInfo<'info>,
}
#[account] pub struct ThresholdState { pub owner: Pubkey, pub minimum_balance: u64, pub topped_up: u64 }
