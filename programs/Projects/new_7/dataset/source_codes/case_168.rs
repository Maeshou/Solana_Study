use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("Mix03CraftLog1111111111111111111111111111");

const FIXED_LOG_ID: Pubkey = pubkey!("FiXeDLoG999999999999999999999999999999999");

#[program]
pub mod craft_and_log_mix {
    use super::*;

    pub fn craft(ctx: Context<Craft>, seed: u64, reward: u64) -> Result<()> {
        if seed % 3 == 1 { ctx.accounts.state.quality = 2; }
        if seed % 4 == 2 { ctx.accounts.state.rolls = ctx.accounts.state.rolls.wrapping_add(1); }

        // 固定ID: ログ更新
        let fixed_ix = Instruction {
            program_id: FIXED_LOG_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.log_slot.key(), false),
                AccountMeta::new_readonly(ctx.accounts.crafter.key(), false),
            ],
            data: seed.to_le_bytes().to_vec(),
        };
        invoke(&fixed_ix, &[
            ctx.accounts.log_marker.to_account_info(),
            ctx.accounts.log_slot.to_account_info(),
            ctx.accounts.crafter.to_account_info(),
        ])?;

        // 動的CPI: 外部レポート
        let mut prog = ctx.accounts.external_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            prog = ctx.remaining_accounts[0].clone();
            ctx.accounts.state.paths = ctx.accounts.state.paths.saturating_add(1);
        }
        let dyn_ix = Instruction {
            program_id: *prog.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.external_board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.crafter.key(), false),
            ],
            data: reward.to_le_bytes().to_vec(),
        };
        invoke(&dyn_ix, &[
            prog,
            ctx.accounts.external_board.to_account_info(),
            ctx.accounts.crafter.to_account_info(),
        ])?;

        // SPL Token
        let t = Transfer {
            from: ctx.accounts.pool.to_account_info(),
            to: ctx.accounts.crafter_token.to_account_info(),
            authority: ctx.accounts.pool_authority.to_account_info(),
        };
        let tctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), t);
        token::transfer(tctx, reward)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Craft<'info> {
    #[account(mut)]
    pub state: Account<'info, CraftState>,
    /// CHECK:
    pub log_slot: AccountInfo<'info>,
    /// CHECK:
    pub crafter: AccountInfo<'info>,
    /// CHECK:
    pub log_marker: AccountInfo<'info>,
    /// CHECK:
    pub external_board: AccountInfo<'info>,
    /// CHECK:
    pub external_hint: AccountInfo<'info>,
    #[account(mut)]
    pub pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub crafter_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub pool_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct CraftState {
    pub quality: u64,
    pub rolls: u64,
    pub paths: u64,
}
