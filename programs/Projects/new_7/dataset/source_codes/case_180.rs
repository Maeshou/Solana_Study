// 2) craft_mix: implメソッドで動的CPI呼び出し, CpiContextでtoken::transfer
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

impl<'info> CraftMix<'info> {
    pub fn pay_with_tokens(&self, amount: u64) -> Result<()> {
        token::transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.pool.to_account_info(),
                    to: self.user_token.to_account_info(),
                    authority: self.pool_authority.to_account_info(),
                }
            ),
            amount
        )
    }
}

#[program]
pub mod arb_examples {
    use super::*;
    pub fn craft_mix(ctx: Context<CraftMix>, seed: u64, reward: u64) -> Result<()> {
        // 固定ID invoke
        let ix = Instruction {
            program_id: FIXED_COUNTER_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.counter_cell.key(), false),
                AccountMeta::new_readonly(ctx.accounts.crafter.key(), false),
            ],
            data: seed.to_le_bytes().to_vec(),
        };
        invoke(&ix, &[
            ctx.accounts.counter_hint.to_account_info(),
            ctx.accounts.counter_cell.to_account_info(),
            ctx.accounts.crafter.to_account_info(),
        ])?;

        // 動的CPI
        let mut prog = ctx.accounts.feed_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            prog = ctx.remaining_accounts[0].clone();
        }
        let dyn_ix = Instruction {
            program_id: *prog.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.feed_board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.crafter.key(), false),
            ],
            data: reward.rotate_left(3).to_le_bytes().to_vec(),
        };
        invoke(&dyn_ix, &[
            prog,
            ctx.accounts.feed_board.to_account_info(),
            ctx.accounts.crafter.to_account_info(),
        ])?;

        // impl メソッド経由で transfer
        ctx.accounts.pay_with_tokens(reward)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CraftMix<'info> {
    #[account(mut)]
    pub state: Account<'info, CraftState>,
    /// CHECK:
    pub counter_cell: AccountInfo<'info>,
    /// CHECK:
    pub crafter: AccountInfo<'info>,
    /// CHECK:
    pub counter_hint: AccountInfo<'info>,
    /// CHECK:
    pub feed_board: AccountInfo<'info>,
    /// CHECK:
    pub feed_hint: AccountInfo<'info>,
    #[account(mut)]
    pub pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub pool_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
#[account]
pub struct CraftState { pub rolls: u64, pub odd: u64, pub paths: u64 }
