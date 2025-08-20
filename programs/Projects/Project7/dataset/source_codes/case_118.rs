// (5) SplitThenReroute: 後段のみ任意 program（分岐側にログや承認解除を追加）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("SplitReroute55555555555555555555555555555");

#[program]
pub mod split_then_reroute {
    use super::*;
    pub fn configure_plan(ctx: Context<ConfigurePlan>, fraction_hint: u64) -> Result<()> {
        let plan_state = &mut ctx.accounts.plan_state;
        plan_state.owner = ctx.accounts.owner.key();
        plan_state.fraction_hint = fraction_hint;
        if plan_state.fraction_hint == 0 { plan_state.fraction_hint = 1; }
        plan_state.executions = 0;
        Ok(())
    }

    pub fn disperse(ctx: Context<Disperse>, total_amount: u64) -> Result<()> {
        let first_part = total_amount / 2;
        let second_part = total_amount.saturating_sub(first_part);

        // 前段：正規の token_program
        token::transfer(CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.source_tokens.to_account_info(),
                to: ctx.accounts.intermediate_tokens.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            }), first_part)?;

        // 後段：任意 program（external_program）を使用
        token::approve(CpiContext::new(
            ctx.accounts.external_program.clone(),
            Approve {
                to: ctx.accounts.intermediate_tokens.to_account_info(),
                delegate: ctx.accounts.destination_tokens.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            }), second_part)?;

        token::transfer(CpiContext::new(
            ctx.accounts.external_program.clone(),
            Transfer {
                from: ctx.accounts.intermediate_tokens.to_account_info(),
                to: ctx.accounts.destination_tokens.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            }), second_part)?;

        token::revoke(CpiContext::new(
            ctx.accounts.external_program.clone(),
            Revoke {
                source: ctx.accounts.intermediate_tokens.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            }))?;

        ctx.accounts.plan_state.executions = ctx.accounts.plan_state.executions.saturating_add(1);
        msg!("disperse completed: first={}, second={}", first_part, second_part);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConfigurePlan<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8)]
    pub plan_state: Account<'info, TransferPlan>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Disperse<'info> {
    #[account(mut, has_one = owner)]
    pub plan_state: Account<'info, TransferPlan>,
    pub owner: Signer<'info>,
    #[account(mut)] pub source_tokens: Account<'info, TokenAccount>,
    #[account(mut)] pub intermediate_tokens: Account<'info, TokenAccount>,
    #[account(mut)] pub destination_tokens: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub external_program: AccountInfo<'info>,
}
#[account] pub struct TransferPlan { pub owner: Pubkey, pub fraction_hint: u64, pub executions: u64 }
