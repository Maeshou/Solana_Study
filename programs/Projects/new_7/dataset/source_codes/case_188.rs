// A) new_with_bytes を使う（Instruction 構築をワンライナー化）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("MiXAltA111111111111111111111111111111111");

const FIXED_ID: Pubkey = pubkey!("FiXeDAAAAA000000000000000000000000000000");

#[program]
pub mod mix_alt_a {
    use super::*;

    pub fn run(ctx: Context<Run>, n: u64, amt: u64) -> Result<()> {
        // 固定ID: new_with_bytes
        invoke(
            &Instruction::new_with_bytes(
                FIXED_ID,
                &n.to_le_bytes(),
                vec![
                    AccountMeta::new(ctx.accounts.fixed_cell.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.actor.key(), false),
                ],
            ),
            &[
                ctx.accounts.fixed_hint.to_account_info(),
                ctx.accounts.fixed_cell.to_account_info(),
                ctx.accounts.actor.to_account_info(),
            ],
        )?;

        // 動的CPI: new_with_bytes + remaining_accounts で program を差し替え
        let mut dyn_prog = ctx.accounts.hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            dyn_prog = ctx.remaining_accounts[0].clone();
            ctx.accounts.book.ticks = ctx.accounts.book.ticks.saturating_add(1);
        }
        invoke(
            &Instruction::new_with_bytes(
                *dyn_prog.key,
                &amt.rotate_left(5).to_le_bytes(),
                vec![
                    AccountMeta::new(ctx.accounts.board.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.actor.key(), false),
                ],
            ),
            &[
                dyn_prog,
                ctx.accounts.board.to_account_info(),
                ctx.accounts.actor.to_account_info(),
            ],
        )?;

        // SPL（安全寄り）
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.user_token.to_account_info(),
                    authority: ctx.accounts.vault_auth.to_account_info(),
                },
            ),
            amt,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Run<'info> {
    #[account(mut)] pub book: Account<'info, Book>,
    /// CHECK: 
    pub fixed_cell: AccountInfo<'info>,
    /// CHECK:
    pub fixed_hint: AccountInfo<'info>,
    /// CHECK:
    pub actor: AccountInfo<'info>,
    /// CHECK:
    pub board: AccountInfo<'info>,
    /// CHECK:
    pub hint: AccountInfo<'info>,
    #[account(mut)] pub vault: Account<'info, TokenAccount>,
    #[account(mut)] pub user_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub vault_auth: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
#[account] pub struct Book { pub ticks: u64 }
