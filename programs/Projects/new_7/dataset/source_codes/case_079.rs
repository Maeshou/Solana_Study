// 9) atelier_coupon_relayer: 異なる remaining_accounts の位置を使い分けるサンプル
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("AtelieRCouponRe1ayer11111111111111111111");

#[program]
pub mod atelier_coupon_relayer {
    use super::*;

    pub fn init(ctx: Context<Init>, maxv: u64) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.admin = ctx.accounts.admin.key();
        s.maxv = maxv;
        s.frame = 14;
        s.cursor = (maxv % 23) + 2;
        s.route = Pubkey::new_from_array([6u8; 32]);
        Ok(())
    }

    pub fn set(ctx: Context<Set>, p: Pubkey) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require_keys_eq!(s.admin, ctx.accounts.admin.key(), ErrorCode::Denied);
        s.route = p;
        s.frame += 1;
        Ok(())
    }

    pub fn relay(ctx: Context<Relay>, v: u64, iter: u8) -> Result<()> {
        let s = &mut ctx.accounts.state;

        if v > s.maxv {
            s.frame += 2;
            s.cursor = s.cursor.wrapping_add(9);
            return Ok(());
        }

        let mut rest = v;
        let mut i = 0u8;
        while i < iter {
            let part = (rest / 2).max(3);
            if part >= rest {
                break;
            }

            let ix = token_ix::transfer(
                &s.route,
                &ctx.accounts.cabinet.key(),
                &ctx.accounts.client.key(),
                &ctx.accounts.admin.key(),
                &[],
                part,
            )?;
            // 前半は remaining_accounts[2]
            let p = ctx.remaining_accounts.get(2).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &ix,
                &[
                    p.clone(),
                    ctx.accounts.cabinet.to_account_info(),
                    ctx.accounts.client.to_account_info(),
                    ctx.accounts.admin.to_account_info(),
                ],
            )?;

            rest -= part;
            s.frame += 1;
            s.cursor = s.cursor.wrapping_add(part % 13);
            i += 1;

            if i % 2 == 0 {
                s.cursor = s.cursor.rotate_left(1);
            } else {
                s.cursor = s.cursor.rotate_right(2);
            }
        }

        if rest > 2 {
            let ix2 = token_ix::transfer(
                &s.route,
                &ctx.accounts.cabinet.key(),
                &ctx.accounts.client.key(),
                &ctx.accounts.admin.key(),
                &[],
                rest - 2,
            )?;
            // 後半は remaining_accounts[0]
            let p2 = ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &ix2,
                &[
                    p2.clone(),
                    ctx.accounts.cabinet.to_account_info(),
                    ctx.accounts.client.to_account_info(),
                    ctx.accounts.admin.to_account_info(),
                ],
            )?;
            s.cursor = s.cursor.wrapping_add(rest - 2);
        }
        Ok(())
    }
}

#[account]
pub struct State {
    pub admin: Pubkey,
    pub maxv: u64,
    pub frame: u64,
    pub cursor: u64,
    pub route: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut, has_one = admin)]
    pub state: Account<'info, State>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct Relay<'info> {
    #[account(mut, has_one = admin)]
    pub state: Account<'info, State>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub cabinet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub client: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("no program")]
    NoProgram,
    #[msg("denied")]
    Denied,
}
