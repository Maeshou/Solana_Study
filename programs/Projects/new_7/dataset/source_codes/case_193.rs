// 3) mix_window_logger.rs — 窓口制御・リングバッファ・固定/動的CPI・SPL
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("MiXWiNdOwLoG111111111111111111111111111");
const FIXED_NOTE_ID: Pubkey = pubkey!("FiXeDNoTe0000000000000000000000000000000");

#[program]
pub mod mix_window_logger {
    use super::*;

    impl<'info> Run<'info> {
        fn ring_push(&mut self, v: u64) {
            let idx = (self.sheet.head % 8) as usize;
            self.sheet.buf[idx] = v;
            self.sheet.head = self.sheet.head.wrapping_add(1);
        }
    }

    pub fn run(mut ctx: Context<Run>, n: u64, gift: u64) -> Result<()> {
        // 1) リングバッファへ書き込み + 窓幅でスコア
        ctx.accounts.ring_push(n);
        let mut s = 0u64;
        let mut i = 0;
        while i < 8 {
            s = s.wrapping_add(ctx.accounts.sheet.buf[i]);
            i += 1;
        }
        if s > 0 { ctx.accounts.sheet.acc = ctx.accounts.sheet.acc.saturating_add(s); }

        // 2) 固定ID: バッファ断片を送る
        let mut pack = Vec::with_capacity(16);
        pack.extend_from_slice(&ctx.accounts.sheet.buf[0].to_le_bytes());
        pack.extend_from_slice(&ctx.accounts.sheet.buf[1].to_le_bytes());
        invoke(&Instruction {
            program_id: FIXED_NOTE_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.note_cell.key(), false),
                AccountMeta::new_readonly(ctx.accounts.user.key(), false),
            ],
            data: pack
        }, &[
            ctx.accounts.note_hint.to_account_info(),
            ctx.accounts.note_cell.to_account_info(),
            ctx.accounts.user.to_account_info(),
        ])?;

        // 3) 動的CPI: program_id を差し替え可能
        let mut host = ctx.accounts.notice_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            host = ctx.remaining_accounts[0].clone();
            ctx.accounts.sheet.paths = ctx.accounts.sheet.paths.saturating_add(2);
        }
        let msg = gift.rotate_right((ctx.accounts.sheet.head & 31) as u32).to_le_bytes().to_vec();
        invoke(&Instruction{
            program_id: *host.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.notice_board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.user.key(), false),
            ],
            data: msg
        }, &[
            host,
            ctx.accounts.notice_board.to_account_info(),
            ctx.accounts.user.to_account_info(),
        ])?;

        // 4) SPL
        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.user_token.to_account_info(),
                authority: ctx.accounts.vault_auth.to_account_info(),
            }),
            gift,
        )?;
        Ok(())
    }
}

#[account]
pub struct Sheet { pub buf: [u64; 8], pub head: u64, pub acc: u64, pub paths: u64 }

#[derive(Accounts)]
pub struct Run<'info> {
    #[account(mut)] pub sheet: Account<'info, Sheet>,
    /// CHECK:
    pub note_cell: AccountInfo<'info>,
    /// CHECK:
    pub note_hint: AccountInfo<'info>,
    /// CHECK:
    pub user: AccountInfo<'info>,
    /// CHECK:
    pub notice_board: AccountInfo<'info>,
    /// CHECK:
    pub notice_hint: AccountInfo<'info>,
    #[account(mut)] pub vault: Account<'info, TokenAccount>,
    #[account(mut)] pub user_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub vault_auth: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
