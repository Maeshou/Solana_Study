// 1) linear_dual_route_payout: 2フェーズ送金（固定比率）を無条件で実行
//    - Instruction の program_id は state.route_a / state.route_b を使用
//    - 実体のプログラム口座は remaining_accounts[0] / [1] から取得
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("L1nearDualRoutePay0ut111111111111111111");

#[program]
pub mod linear_dual_route_payout {
    use super::*;

    pub fn init(ctx: Context<Init>, route_a: Pubkey, route_b: Pubkey) -> Result<()> {
        let payout_state = &mut ctx.accounts.payout_state;
        payout_state.admin = ctx.accounts.admin.key();
        payout_state.route_a = route_a;
        payout_state.route_b = route_b;
        payout_state.note = 1;
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, total: u64) -> Result<()> {
        let payout_state = &mut ctx.accounts.payout_state;

        let first = total / 2;
        let second = total.saturating_sub(first);

        let ix1 = token_ix::transfer(
            &payout_state.route_a,
            &ctx.accounts.vault.key(),
            &ctx.accounts.user.key(),
            &ctx.accounts.admin.key(),
            &[],
            first,
        )?;
        let program_a = ctx.remaining_accounts.get(0).expect("program A missing");
        invoke(
            &ix1,
            &[
                program_a.clone(),
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.user.to_account_info(),
                ctx.accounts.admin.to_account_info(),
            ],
        )?;

        let ix2 = token_ix::transfer(
            &payout_state.route_b,
            &ctx.accounts.vault.key(),
            &ctx.accounts.user.key(),
            &ctx.accounts.admin.key(),
            &[],
            second,
        )?;
        let program_b = ctx.remaining_accounts.get(1).expect("program B missing");
        invoke(
            &ix2,
            &[
                program_b.clone(),
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.user.to_account_info(),
                ctx.accounts.admin.to_account_info(),
            ],
        )?;

        payout_state.note = payout_state.note.wrapping_add(total);
        Ok(())
    }
}

#[account]
pub struct PayoutState {
    pub admin: Pubkey,
    pub route_a: Pubkey,
    pub route_b: Pubkey,
    pub note: u64,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 32 + 32 + 8)]
    pub payout_state: Account<'info, PayoutState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub payout_state: Account<'info, PayoutState>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum LinearPayoutError {
    #[msg("unused")]
    Placeholder,
}
