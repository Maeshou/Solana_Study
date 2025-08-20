// 2) guild_tip_pipeline: remaining_accounts[1] をプログラムに使う派生ルート＋分岐の内側を厚めに
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("Gu1ldT1pP1pe1ine111111111111111111111111");

#[program]
pub mod guild_tip_pipeline {
    use super::*;

    pub fn init(ctx: Context<Init>, cap: u64) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.master = ctx.accounts.master.key();
        s.cap = cap;
        s.steps = 5;
        s.acc = (cap ^ 0xA5A5) as u64;
        s.route = Pubkey::new_unique();
        Ok(())
    }

    pub fn rebind(ctx: Context<Rebind>, new_route: Pubkey) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require_keys_eq!(s.master, ctx.accounts.master.key(), ErrorCode::Denied);
        s.route = new_route;
        s.steps += 3;
        s.acc = s.acc.rotate_left(3);
        Ok(())
    }

    pub fn stream(ctx: Context<Stream>, value: u64, loops: u8) -> Result<()> {
        let s = &mut ctx.accounts.state;

        if value <= 3 {
            s.steps += 1;
            s.acc = s.acc.wrapping_add(19);
            return Ok(());
        }

        let mut rest = value;
        let mut i = 0u8;
        while i < loops {
            // 3分割の一部を送る
            let part = (rest / 3).max(4);
            if part >= rest {
                break;
            }

            let ix = token_ix::transfer(
                &s.route,
                &ctx.accounts.source.key(),
                &ctx.accounts.sink.key(),
                &ctx.accounts.master.key(),
                &[],
                part,
            )?;

            // 実体は remaining_accounts[1]
            let prog = ctx.remaining_accounts.get(1).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &ix,
                &[
                    prog.clone(),
                    ctx.accounts.source.to_account_info(),
                    ctx.accounts.sink.to_account_info(),
                    ctx.accounts.master.to_account_info(),
                ],
            )?;

            rest -= part;
            s.steps += 1;
            i += 1;

            // ループ内後処理：擬似メトリクス更新
            if rest < s.cap / 3 {
                s.acc = s.acc.wrapping_add(part ^ 17);
            } else {
                s.acc = s.acc.wrapping_mul(3).wrapping_add(23);
            }
        }

        if rest > 3 {
            let ix2 = token_ix::transfer(
                &s.route,
                &ctx.accounts.source.key(),
                &ctx.accounts.sink.key(),
                &ctx.accounts.master.key(),
                &[],
                rest - 3,
            )?;
            let prog = ctx.remaining_accounts.get(1).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &ix2,
                &[
                    prog.clone(),
                    ctx.accounts.source.to_account_info(),
                    ctx.accounts.sink.to_account_info(),
                    ctx.accounts.master.to_account_info(),
                ],
            )?;
            s.acc = s.acc.wrapping_add(rest - 3);
        }

        // 仕上げの微調整ループ
        let mut t = 1u8;
        while t < 3 {
            s.steps += 2;
            s.acc = s.acc.rotate_right(t as u32);
            t += 1;
        }
        Ok(())
    }
}

#[account]
pub struct State {
    pub master: Pubkey,
    pub cap: u64,
    pub steps: u64,
    pub acc: u64,
    pub route: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = master, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub master: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Rebind<'info> {
    #[account(mut, has_one = master)]
    pub state: Account<'info, State>,
    pub master: Signer<'info>,
}

#[derive(Accounts)]
pub struct Stream<'info> {
    #[account(mut, has_one = master)]
    pub state: Account<'info, State>,
    pub master: Signer<'info>,
    #[account(mut)]
    pub source: Account<'info, TokenAccount>,
    #[account(mut)]
    pub sink: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("not allowed")]
    Denied,
    #[msg("program account missing")]
    NoProgram,
}
