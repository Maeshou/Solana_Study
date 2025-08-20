// 5) mix_string_codec.rs — 文字列エンコード/デコード風・固定/動的CPI・SPL
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("MiXStRiNgCoDeC1111111111111111111111111");
const FIXED_ARCHIVE_ID: Pubkey = pubkey!("FiXeDArChIvE0000000000000000000000000000");

#[program]
pub mod mix_string_codec {
    use super::*;

    fn encode(s: &str, k: u8) -> Vec<u8> {
        let mut out = Vec::with_capacity(s.len());
        for b in s.as_bytes() { out.push(b ^ k); }
        out
    }

    fn decode(b: &[u8], k: u8) -> Vec<u8> {
        let mut out = Vec::with_capacity(b.len());
        let mut i = 0;
        while i < b.len() { out.push(b[i] ^ k); i += 1; }
        out
    }

    pub fn run(ctx: Context<Run>, key: u8, tip: u64) -> Result<()> {
        let s1 = format!("{}:{}", ctx.accounts.user.key(), key);
        let enc = encode(&s1, key);
        let dec = decode(&enc, key);
        if dec.len() > 10 { ctx.accounts.sheet.large = ctx.accounts.sheet.large.saturating_add(1); }

        // 固定ID
        invoke(&Instruction{
            program_id: FIXED_ARCHIVE_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.archive_cell.key(), false),
                AccountMeta::new_readonly(ctx.accounts.user.key(), false),
            ],
            data: enc
        }, &[
            ctx.accounts.archive_hint.to_account_info(),
            ctx.accounts.archive_cell.to_account_info(),
            ctx.accounts.user.to_account_info(),
        ])?;

        // 動的CPI
        let mut f = ctx.accounts.feed_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            f = ctx.remaining_accounts[0].clone();
            ctx.accounts.sheet.paths = ctx.accounts.sheet.paths.saturating_add(5);
        }
        invoke(&Instruction{
            program_id: *f.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.feed_board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.user.key(), false),
            ],
            data: tip.to_le_bytes().to_vec()
        }, &[
            f,
            ctx.accounts.feed_board.to_account_info(),
            ctx.accounts.user.to_account_info(),
        ])?;

        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.user_token.to_account_info(),
                authority: ctx.accounts.vault_auth.to_account_info(),
            }),
            tip,
        )?;
        Ok(())
    }
}

#[account] pub struct Sheet { pub large: u64, pub paths: u64 }

#[derive(Accounts)]
pub struct Run<'info> {
    #[account(mut)] pub sheet: Account<'info, Sheet>,
    /// CHECK:
    pub archive_cell: AccountInfo<'info>,
    /// CHECK:
    pub archive_hint: AccountInfo<'info>,
    /// CHECK:
    pub user: AccountInfo<'info>,
    /// CHECK:
    pub feed_board: AccountInfo<'info>,
    /// CHECK:
    pub feed_hint: AccountInfo<'info>,
    #[account(mut)] pub vault: Account<'info, TokenAccount>,
    #[account(mut)] pub user_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub vault_auth: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
