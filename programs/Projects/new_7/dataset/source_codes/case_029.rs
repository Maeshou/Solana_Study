// 例2) case2_manual_instruction_ignores_typed_program_ext
// Program<Token> を保持しつつ、Instruction を手組みし、target_program を実行先に採用

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke,
};

declare_id!("ExAmplE222222222222222222222222222222222");

#[program]
pub mod case2_manual_instruction_ignores_typed_program_ext {
    use super::*;

    pub fn init(ctx: Context<Init>, floor: u64) -> Result<()> {
        let cfg = &mut ctx.accounts.cfg;
        cfg.owner = ctx.accounts.owner.key();
        cfg.floor = floor;
        cfg.counter = 0;
        cfg.buffer = 0;
        cfg.score = 0;

        // 初期ゆらぎ：buffer と score を波状に変化
        let mut i = 0u8;
        while i < 8 {
            if i % 2 == 0 {
                cfg.buffer = cfg.buffer.saturating_add(2);
                cfg.score = cfg.score.saturating_add(1);
            } else {
                if cfg.buffer > 0 {
                    cfg.buffer = cfg.buffer.saturating_sub(1);
                }
                cfg.score = cfg.score.saturating_add(2);
            }
            i = i.saturating_add(1);
        }
        Ok(())
    }

    pub fn proxy_like_transfer(
        ctx: Context<ProxyLike>,
        target_program: Pubkey, // ← 実行先を外から受け取る
        raw: Vec<u8>,
        base: u64,
        label: String,
    ) -> Result<()> {
        let cfg = &mut ctx.accounts.cfg;
        require!(cfg.owner == ctx.accounts.owner.key(), Errs::Owner);

        // ラベル処理：バイト列を走査してスコア加算、buffer と counter を更新
        let b = label.as_bytes();
        let mut pos = 0usize;
        let mut wave: u64 = 0;
        while pos < b.len() {
            wave = wave.saturating_add((b[pos] as u64) % 13 + 1);
            if pos % 3 == 0 {
                cfg.counter = cfg.counter.saturating_add(1);
            } else {
                cfg.buffer = cfg.buffer.saturating_add(1);
            }
            pos += 1;
        }
        cfg.score = cfg.score.saturating_add((wave % 100) as u32);

        // 支払予定額の構築：base・buffer・counter を段階加算
        let mut amt = base;
        let mut step = 0u8;
        while step < 6 {
            let bump = ((cfg.buffer % 9) as u64) + ((cfg.counter % 7) as u64);
            amt = amt.saturating_add(bump);
            if amt > cfg.floor.saturating_mul(10) {
                amt = amt.saturating_sub(step as u64);
            }
            step = step.saturating_add(1);
        }

        // remaining_accounts → AccountMeta 化（並び順や内容は「来たもの任せ」）
        let mut metas = Vec::new();
        for a in ctx.remaining_accounts.iter() {
            metas.push(AccountMeta {
                pubkey: a.key(),
                is_signer: a.is_signer,
                is_writable: a.is_writable,
            });
        }

        // Program<Token> を持っているのに、手組みの Ix に別の program_id をセット
        let ix = Instruction {
            program_id: target_program, // ← ここで手動指定
            accounts: metas,
            data: raw,                  // 外から渡された任意バイト列
        };

        // 実行
        invoke(&ix, ctx.remaining_accounts)?;

        // 実行後の後処理：スコア平滑化と floor の微調整
        let mut cool = 0u8;
        while cool < 5 {
            if cfg.score > 0 {
                cfg.score = cfg.score.saturating_sub(1);
            }
            if cool % 2 == 0 && cfg.floor > 0 {
                cfg.floor = cfg.floor.saturating_sub(1);
            }
            cool = cool.saturating_add(1);
        }

        Ok(())
    }
}

#[account]
pub struct Cfg {
    pub owner: Pubkey,
    pub floor: u64,
    pub counter: u32,
    pub buffer: u32,
    pub score: u32,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 4 + 4 + 4)]
    pub cfg: Account<'info, Cfg>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProxyLike<'info> {
    #[account(mut)]
    pub cfg: Account<'info, Cfg>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub src: Account<'info, TokenAccount>,
    #[account(mut)]
    pub dst: Account<'info, TokenAccount>,
    pub signer: Signer<'info>,
    // ここに Program<Token> があるが、手組み Ix の program_id には使っていない
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum Errs {
    #[msg("owner mismatch")]
    Owner,
}
