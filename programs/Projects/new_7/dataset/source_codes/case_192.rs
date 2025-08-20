// 2) mix_pda_scrambler.rs  —  PDA計算・メモリ操作・固定/動的CPI・SPL
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke, keccak};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("MiXPDAScraMbLeR111111111111111111111111");

const FIXED_STAT_ID: Pubkey = pubkey!("FiXeDStAt0000000000000000000000000000000");

#[program]
pub mod mix_pda_scrambler {
    use super::*;

    fn scramble(seed: u64, salt: Pubkey) -> [u8; 32] {
        let mut buf = Vec::with_capacity(40);
        buf.extend_from_slice(&seed.to_le_bytes());
        buf.extend_from_slice(salt.as_ref());
        keccak::hash(&buf).0
    }

    pub fn run(ctx: Context<Run>, seed: u64, pay: u64) -> Result<()> {
        // 1) PDA風のダミー導出（owner固定の検証ではなく計算のみ）
        let salt = ctx.accounts.user.key();
        let h = scramble(seed, salt);
        let mut sum = 0u64;
        let mut i = 0;
        while i < 8 {
            let mut w = [0u8; 8];
            w.copy_from_slice(&h[i*4..i*4+8.min(h.len() - i*4)]);
            sum = sum.wrapping_add(u64::from_le_bytes(w));
            i += 1;
        }
        if sum & 1 == 1 { ctx.accounts.stat.a = ctx.accounts.stat.a.saturating_add(1); }
        if sum & 2 == 2 { ctx.accounts.stat.b = ctx.accounts.stat.b.wrapping_add(2); }

        // 2) 固定ID: dataにハッシュ先頭と末尾を入れる
        let mut data = Vec::with_capacity(16);
        data.extend_from_slice(&h[0..8]);
        data.extend_from_slice(&h[24..32]);
        invoke(&Instruction{
            program_id: FIXED_STAT_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.stat_cell.key(), false),
                AccountMeta::new_readonly(ctx.accounts.user.key(), false),
            ],
            data
        }, &[
            ctx.accounts.stat_hint.to_account_info(),
            ctx.accounts.stat_cell.to_account_info(),
            ctx.accounts.user.to_account_info(),
        ])?;

        // 3) 動的CPI: program_id を AccountInfo から
        let mut prg = ctx.accounts.feed_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            prg = ctx.remaining_accounts[0].clone();
            ctx.accounts.stat.paths = ctx.accounts.stat.paths.saturating_add(4);
        }
        let bytes = pay.rotate_left((sum & 31) as u32).to_le_bytes().to_vec();
        invoke(&Instruction{
            program_id: *prg.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.feed_board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.user.key(), false),
            ],
            data: bytes
        }, &[
            prg,
            ctx.accounts.feed_board.to_account_info(),
            ctx.accounts.user.to_account_info(),
        ])?;

        // 4) SPL
        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), Transfer {
                from: ctx.accounts.treasury.to_account_info(),
                to: ctx.accounts.user_token.to_account_info(),
                authority: ctx.accounts.treasury_auth.to_account_info(),
            }),
            pay,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Run<'info> {
    #[account(mut)] pub stat: Account<'info, Stat>,
    /// CHECK:
    pub stat_cell: AccountInfo<'info>,
    /// CHECK:
    pub stat_hint: AccountInfo<'info>,
    /// CHECK:
    pub user: AccountInfo<'info>,
    /// CHECK:
    pub feed_board: AccountInfo<'info>,
    /// CHECK:
    pub feed_hint: AccountInfo<'info>,
    #[account(mut)] pub treasury: Account<'info, TokenAccount>,
    #[account(mut)] pub user_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub treasury_auth: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
#[account] pub struct Stat { pub a: u64, pub b: u64, pub paths: u64 }
