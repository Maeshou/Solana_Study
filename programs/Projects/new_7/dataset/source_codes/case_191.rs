// 1) mix_event_meter.rs
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke, sysvar::clock::Clock};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("MiXEvEnTMeteR11111111111111111111111111");
const FIXED_LOG_ID: Pubkey = pubkey!("FiXeDLoG00000000000000000000000000000000");

#[event]
pub struct Milestone { pub slot: u64, pub level: u64, pub tag: u64 }

#[program]
pub mod mix_event_meter {
    use super::*;

    fn fold_bits(v: u64) -> u64 {
        let a = v.rotate_left(3) ^ v.rotate_right(5);
        let b = a.wrapping_mul(911);
        b ^ (b >> 7)
    }

    pub fn run(ctx: Context<Run>, mark: u64, tip: u64) -> Result<()> {
        // 1) 計算: スライディング風メータ + Sysvar
        let now = Clock::get()?.slot;
        let f1 = fold_bits(mark);
        let f2 = fold_bits(now);
        let meter = f1.wrapping_add(f2);
        if meter & 1 == 1 { ctx.accounts.state.odd = ctx.accounts.state.odd.saturating_add(1); }
        if meter & 2 == 2 { ctx.accounts.state.bump = ctx.accounts.state.bump.wrapping_add(2); }

        // 2) 固定ID: 文字列を組み立て→長さとXORをdataに載せる
        let mut msg = mark.to_string();
        msg.push('-');
        msg.push_str(&now.to_string());
        let mut x = 0u8;
        for b in msg.as_bytes() { x ^= *b; }
        let mut data = msg.into_bytes();
        data.push(x);

        let metas = vec![
            AccountMeta::new(ctx.accounts.log_cell.key(), false),
            AccountMeta::new_readonly(ctx.accounts.user.key(), false),
        ];
        invoke(&Instruction{ program_id: FIXED_LOG_ID, accounts: metas, data }, &[
            ctx.accounts.log_hint.to_account_info(),
            ctx.accounts.log_cell.to_account_info(),
            ctx.accounts.user.to_account_info(),
        ])?;

        // 3) 動的CPI: remaining_accounts から差し替え + 配列並べ替えをデータ化
        let mut p = ctx.accounts.dynamic_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            p = ctx.remaining_accounts[0].clone();
            ctx.accounts.state.paths = ctx.accounts.state.paths.saturating_add(3);
        }
        let mut arr = [tip, tip.rotate_left(9), tip ^ 777];
        // 簡易バブル: 降順（順不同実装）
        let mut i = 0;
        while i < 3 {
            let mut j = 1;
            while j < 3 {
                if arr[j - 1] < arr[j] {
                    let t = arr[j - 1]; arr[j - 1] = arr[j]; arr[j] = t;
                }
                j += 1;
            }
            i += 1;
        }
        let mut dynbytes = Vec::new();
        for v in arr { dynbytes.extend_from_slice(&v.to_le_bytes()); }
        invoke(&Instruction{
            program_id: *p.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.dynamic_board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.user.key(), false),
            ],
            data: dynbytes,
        }, &[
            p,
            ctx.accounts.dynamic_board.to_account_info(),
            ctx.accounts.user.to_account_info(),
        ])?;

        // 4) イベント発火
        emit!(Milestone{ slot: now, level: meter & 15, tag: mark.rotate_right(3) });

        // 5) SPL 送金
        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), Transfer {
                from: ctx.accounts.bank.to_account_info(),
                to: ctx.accounts.user_token.to_account_info(),
                authority: ctx.accounts.bank_auth.to_account_info(),
            }),
            tip,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Run<'info> {
    #[account(mut)] pub state: Account<'info, Meter>,
    /// CHECK: 
    pub log_cell: AccountInfo<'info>,
    /// CHECK:
    pub log_hint: AccountInfo<'info>,
    /// CHECK:
    pub user: AccountInfo<'info>,
    /// CHECK:
    pub dynamic_board: AccountInfo<'info>,
    /// CHECK:
    pub dynamic_hint: AccountInfo<'info>,
    #[account(mut)] pub bank: Account<'info, TokenAccount>,
    #[account(mut)] pub user_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub bank_auth: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
#[account] pub struct Meter { pub odd: u64, pub bump: u64, pub paths: u64 }
