// 6) mix_counter_sieve.rs — 簡易ふるい・素数っぽいカウント・固定/動的CPI・SPL
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("MiXSiEvECouNt11111111111111111111111111");
const FIXED_BOOK_ID: Pubkey = pubkey!("FiXeDBooK0000000000000000000000000000000");

#[program]
pub mod mix_counter_sieve {
    use super::*;

    fn count_like_primes(n: u64) -> u64 {
        let mut c = 0u64;
        let mut i = 2u64;
        while i <= n {
            let mut d = 2u64;
            let mut ok = true;
            while d * d <= i {
                if i % d == 0 { ok = false; }
                d += 1;
            }
            if ok { c = c.saturating_add(1); }
            i += 1;
        }
        c
    }

    pub fn run(ctx: Context<Run>, n: u64, reward: u64) -> Result<()> {
        let k = count_like_primes(n);
        if k > 0 { ctx.accounts.book.primes = ctx.accounts.book.primes.saturating_add(k); }

        // 固定ID
        let blob = k.to_le_bytes().to_vec();
        invoke(&Instruction{
            program_id: FIXED_BOOK_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.book_cell.key(), false),
                AccountMeta::new_readonly(ctx.accounts.user.key(), false),
            ],
            data: blob
        }, &[
            ctx.accounts.book_hint.to_account_info(),
            ctx.accounts.book_cell.to_account_info(),
            ctx.accounts.user.to_account_info(),
        ])?;

        // 動的CPI
        let mut nprg = ctx.accounts.notice_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            nprg = ctx.remaining_accounts[0].clone();
            ctx.accounts.book.paths = ctx.accounts.book.paths.saturating_add(2);
        }
        invoke(&Instruction{
            program_id: *nprg.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.notice_board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.user.key(), false),
            ],
            data: reward.rotate_left((k & 31) as u32).to_le_bytes().to_vec()
        }, &[
            nprg,
            ctx.accounts.notice_board.to_account_info(),
            ctx.accounts.user.to_account_info(),
        ])?;

        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), Transfer {
                from: ctx.accounts.bank.to_account_info(),
                to: ctx.accounts.user_token.to_account_info(),
                authority: ctx.accounts.bank_auth.to_account_info(),
            }),
            reward,
        )?;
        Ok(())
    }
}

#[account] pub struct Book { pub primes: u64, pub paths: u64 }

#[derive(Accounts)]
pub struct Run<'info> {
    #[account(mut)] pub book: Account<'info, Book>,
    /// CHECK:
    pub book_cell: AccountInfo<'info>,
    /// CHECK:
    pub book_hint: AccountInfo<'info>,
    /// CHECK:
    pub user: AccountInfo<'info>,
    /// CHECK:
    pub notice_board: AccountInfo<'info>,
    /// CHECK:
    pub notice_hint: AccountInfo<'info>,
    #[account(mut)] pub bank: Account<'info, TokenAccount>,
    #[account(mut)] pub user_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub bank_auth: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
