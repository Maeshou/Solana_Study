// 6) palette_color_airdrop: 2段階の分岐と後処理ループを追加、Instruction 側 program_id を状態から
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("Pa1ett3ColorAirdrop11111111111111111111");

#[program]
pub mod palette_color_airdrop {
    use super::*;

    pub fn init(ctx: Context<Init>, cap: u64) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.admin = ctx.accounts.admin.key();
        s.cap = cap;
        s.hist = 10;
        s.gain = 1u64.wrapping_add(cap % 17);
        s.route = Pubkey::new_from_array([3u8; 32]);
        Ok(())
    }

    pub fn route_to(ctx: Context<RouteTo>, pid: Pubkey) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require_keys_eq!(s.admin, ctx.accounts.admin.key(), ErrorCode::Denied);
        s.route = pid;
        s.hist = s.hist.wrapping_add(2);
        Ok(())
    }

    pub fn airdrop(ctx: Context<Airdrop>, drop: u64, rounds: u8) -> Result<()> {
        let s = &mut ctx.accounts.state;
        if drop < 6 {
            s.hist += 3;
            s.gain = s.gain.rotate_left(1);
            return Ok(());
        }

        let mut rest = drop;
        let mut turn = 0u8;

        while turn < rounds {
            let chunk = (rest / 5).max(2);
            if chunk >= rest {
                break;
            }

            let ix = token_ix::transfer(
                &s.route,
                &ctx.accounts.palette_bank.key(),
                &ctx.accounts.artist_wallet.key(),
                &ctx.accounts.admin.key(),
                &[],
                chunk,
            )?;
            let program_ai = ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &ix,
                &[
                    program_ai.clone(),
                    ctx.accounts.palette_bank.to_account_info(),
                    ctx.accounts.artist_wallet.to_account_info(),
                    ctx.accounts.admin.to_account_info(),
                ],
            )?;

            rest -= chunk;
            s.hist += 1;
            s.gain = s.gain.wrapping_add(chunk ^ 29);
            turn += 1;

            if s.gain % 3 == 0 {
                s.gain = s.gain.wrapping_add(13);
            } else {
                s.gain = s.gain.wrapping_sub(2).wrapping_add(21);
            }
        }

        if rest > 3 {
            let ix2 = token_ix::transfer(
                &s.route,
                &ctx.accounts.palette_bank.key(),
                &ctx.accounts.artist_wallet.key(),
                &ctx.accounts.admin.key(),
                &[],
                rest - 3,
            )?;
            let program_ai = ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &ix2,
                &[
                    program_ai.clone(),
                    ctx.accounts.palette_bank.to_account_info(),
                    ctx.accounts.artist_wallet.to_account_info(),
                    ctx.accounts.admin.to_account_info(),
                ],
            )?;
            s.gain = s.gain.wrapping_add(rest - 3);
        }

        let mut j = 1u8;
        while j < 4 {
            s.hist += 1;
            s.gain = s.gain.rotate_right(j as u32);
            j += 1;
        }
        Ok(())
    }
}

#[account]
pub struct State {
    pub admin: Pubkey,
    pub cap: u64,
    pub hist: u64,
    pub gain: u64,
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
pub struct RouteTo<'info> {
    #[account(mut, has_one = admin)]
    pub state: Account<'info, State>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct Airdrop<'info> {
    #[account(mut, has_one = admin)]
    pub state: Account<'info, State>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub palette_bank: Account<'info, TokenAccount>,
    #[account(mut)]
    pub artist_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("no program")]
    NoProgram,
    #[msg("denied")]
    Denied,
}
