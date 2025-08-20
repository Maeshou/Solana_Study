// 4) mix_checksum_router.rs — チェックサム・並べ替え・固定/動的CPI・SPL
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("MiXChEcKoUtEr11111111111111111111111111");
const FIXED_SUM_ID: Pubkey = pubkey!("FiXeDSuM00000000000000000000000000000000");

#[program]
pub mod mix_checksum_router {
    use super::*;

    fn checksum(mut v: u64) -> u64 {
        let mut s = 0u64;
        let mut i = 0;
        while i < 8 {
            s ^= v & 0xff;
            v >>= 8;
            i += 1;
        }
        s
    }

    pub fn run(ctx: Context<Run>, a: u64, b: u64, pay: u64) -> Result<()> {
        // 1) 整列＆チェックサム
        let mut x = [a, b, a ^ b];
        if x[0] > x[1] { let t=x[0]; x[0]=x[1]; x[1]=t; }
        if x[1] > x[2] { let t=x[1]; x[1]=x[2]; x[2]=t; }
        if x[0] > x[1] { let t=x[0]; x[0]=x[1]; x[1]=t; }

        let mut s = 0u64;
        let mut i = 0;
        while i < 3 { s = s.wrapping_add(checksum(x[i])); i += 1; }
        if s & 1 == 1 { ctx.accounts.tally.low = ctx.accounts.tally.low.saturating_add(1); }
        if s & 2 == 2 { ctx.accounts.tally.high = ctx.accounts.tally.high.wrapping_add(2); }

        // 2) 固定ID
        let mut bytes = Vec::with_capacity(24);
        for v in x { bytes.extend_from_slice(&v.to_le_bytes()); }
        invoke(&Instruction{
            program_id: FIXED_SUM_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.sum_cell.key(), false),
                AccountMeta::new_readonly(ctx.accounts.user.key(), false),
            ],
            data: bytes
        }, &[
            ctx.accounts.sum_hint.to_account_info(),
            ctx.accounts.sum_cell.to_account_info(),
            ctx.accounts.user.to_account_info(),
        ])?;

        // 3) 動的CPI
        let mut p = ctx.accounts.router_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            p = ctx.remaining_accounts[0].clone();
            ctx.accounts.tally.paths = ctx.accounts.tally.paths.saturating_add(1);
        }
        let msg = pay.rotate_left((s & 15) as u32).to_le_bytes().to_vec();
        invoke(&Instruction{
            program_id: *p.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.router_board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.user.key(), false),
            ],
            data: msg
        }, &[
            p,
            ctx.accounts.router_board.to_account_info(),
            ctx.accounts.user.to_account_info(),
        ])?;

        // 4) SPL
        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), Transfer {
                from: ctx.accounts.bank.to_account_info(),
                to: ctx.accounts.user_token.to_account_info(),
                authority: ctx.accounts.bank_auth.to_account_info(),
            }),
            pay,
        )?;
        Ok(())
    }
}

#[account] pub struct Tally { pub low: u64, pub high: u64, pub paths: u64 }

#[derive(Accounts)]
pub struct Run<'info> {
    #[account(mut)] pub tally: Account<'info, Tally>,
    /// CHECK:
    pub sum_cell: AccountInfo<'info>,
    /// CHECK:
    pub sum_hint: AccountInfo<'info>,
    /// CHECK:
    pub user: AccountInfo<'info>,
    /// CHECK:
    pub router_board: AccountInfo<'info>,
    /// CHECK:
    pub router_hint: AccountInfo<'info>,
    #[account(mut)] pub bank: Account<'info, TokenAccount>,
    #[account(mut)] pub user_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub bank_auth: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
