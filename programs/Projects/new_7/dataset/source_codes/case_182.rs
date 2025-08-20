// 1) quest_route_mix.rs
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("QuEsTRouTeMiX111111111111111111111111111");

const FIXED_PING_ID: Pubkey = pubkey!("FiXeDP1nG0000000000000000000000000000000");

#[program]
pub mod quest_route_mix {
    use super::*;

    fn pay_reward(tp: &Program<Token>, from: &Account<TokenAccount>, to: &Account<TokenAccount>, auth: &AccountInfo, amt: u64) -> Result<()> {
        token::transfer(
            CpiContext::new(
                tp.to_account_info(),
                Transfer { from: from.to_account_info(), to: to.to_account_info(), authority: auth.clone() }
            ),
            amt
        )
    }

    pub fn act(ctx: Context<Act>, stage: u64, reward: u64) -> Result<()> {
        if stage > 3 {
            ctx.accounts.local.note = ctx.accounts.local.note.saturating_add(stage);
        }

        // 固定ID（安全寄り）
        let ix_fixed = Instruction {
            program_id: FIXED_PING_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.slot_cell.key(), false),
                AccountMeta::new_readonly(ctx.accounts.runner.key(), false),
            ],
            data: stage.to_le_bytes().to_vec(),
        };
        invoke(&ix_fixed, &[
            ctx.accounts.slot_hint.to_account_info(),
            ctx.accounts.slot_cell.to_account_info(),
            ctx.accounts.runner.to_account_info(),
        ])?;

        // 動的CPI（差し替え可能）
        let mut host = ctx.accounts.board_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            host = ctx.remaining_accounts[0].clone();
            ctx.accounts.local.paths = ctx.accounts.local.paths.wrapping_add(1);
        }
        let ix_dyn = Instruction {
            program_id: *host.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.board_pad.key(), false),
                AccountMeta::new_readonly(ctx.accounts.runner.key(), false),
            ],
            data: reward.rotate_left(3).to_le_bytes().to_vec(),
        };
        invoke(&ix_dyn, &[
            host,
            ctx.accounts.board_pad.to_account_info(),
            ctx.accounts.runner.to_account_info(),
        ])?;

        // SPL Token 送金（ID固定）
        pay_reward(&ctx.accounts.token_program, &ctx.accounts.treasury, &ctx.accounts.user_token, &ctx.accounts.treasury_authority, reward)
    }
}

#[derive(Accounts)]
pub struct Act<'info> {
    #[account(mut)]
    pub local: Account<'info, LocalState>,
    /// CHECK:
    pub slot_cell: AccountInfo<'info>,
    /// CHECK:
    pub runner: AccountInfo<'info>,
    /// CHECK:
    pub slot_hint: AccountInfo<'info>,
    /// CHECK:
    pub board_pad: AccountInfo<'info>,
    /// CHECK:
    pub board_hint: AccountInfo<'info>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub treasury_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
#[account]
pub struct LocalState { pub note: u64, pub paths: u64 }
