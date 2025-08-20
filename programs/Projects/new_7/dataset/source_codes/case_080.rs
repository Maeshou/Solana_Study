// 10) arcade_ticket_payout: 途中で補正ロジックを複数入れ、最後に一括送付
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("Arcad3T1cketPay0ut1111111111111111111111");

#[program]
pub mod arcade_ticket_payout {
    use super::*;

    pub fn init(ctx: Context<Init>, hard: u64) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.operator = ctx.accounts.operator.key();
        s.hard = hard;
        s.round = 9;
        s.note = hard.rotate_right(3);
        s.prog_id = Pubkey::new_from_array([2u8; 32]);
        Ok(())
    }

    pub fn switch(ctx: Context<Switch>, pid: Pubkey) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require_keys_eq!(s.operator, ctx.accounts.operator.key(), ErrorCode::Denied);
        s.prog_id = pid;
        s.round = s.round.wrapping_add(6);
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, tickets: u64, times: u8) -> Result<()> {
        let s = &mut ctx.accounts.state;

        if tickets < 8 {
            s.round += 2;
            s.note = s.note.wrapping_add(100);
            return Ok(());
        }

        let mut rest = tickets;
        let mut i = 0u8;
        let mut checksum = 17u64;

        while i < times {
            let step = (rest / 3).max(4);
            if step >= rest {
                break;
            }

            let ix = token_ix::transfer(
                &s.prog_id,
                &ctx.accounts.reserve.key(),
                &ctx.accounts.gamer.key(),
                &ctx.accounts.operator.key(),
                &[],
                step,
            )?;
            let p = ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &ix,
                &[
                    p.clone(),
                    ctx.accounts.reserve.to_account_info(),
                    ctx.accounts.gamer.to_account_info(),
                    ctx.accounts.operator.to_account_info(),
                ],
            )?;

            rest -= step;
            i += 1;
            s.round += 1;

            // チェックサムとメモ更新
            checksum = checksum.wrapping_add(step ^ 0xAA);
            if checksum % 5 == 0 {
                s.note = s.note.wrapping_add(checksum);
            } else {
                s.note = s.note.wrapping_sub(13).wrapping_add(3);
            }

            if rest <= s.hard / 3 {
                s.note = s.note.rotate_left(2);
            } else {
                s.note = s.note.rotate_right(1);
            }
        }

        if rest > 5 {
            let ix2 = token_ix::transfer(
                &s.prog_id,
                &ctx.accounts.reserve.key(),
                &ctx.accounts.gamer.key(),
                &ctx.accounts.operator.key(),
                &[],
                rest - 5,
            )?;
            let p = ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &ix2,
                &[
                    p.clone(),
                    ctx.accounts.reserve.to_account_info(),
                    ctx.accounts.gamer.to_account_info(),
                    ctx.accounts.operator.to_account_info(),
                ],
            )?;
            s.note = s.note.wrapping_add(rest - 5);
        }

        // 後処理：巻き戻し風の補正
        let mut z = 1u8;
        while z < 4 {
            s.round += 1;
            s.note = s.note.rotate_right(z as u32).wrapping_add(7);
            z += 1;
        }
        Ok(())
    }
}

#[account]
pub struct State {
    pub operator: Pubkey,
    pub hard: u64,
    pub round: u64,
    pub note: u64,
    pub prog_id: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Switch<'info> {
    #[account(mut, has_one = operator)]
    pub state: Account<'info, State>,
    pub operator: Signer<'info>,
}

#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut, has_one = operator)]
    pub state: Account<'info, State>,
    pub operator: Signer<'info>,
    #[account(mut)]
    pub reserve: Account<'info, TokenAccount>,
    #[account(mut)]
    pub gamer: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("denied")]
    Denied,
    #[msg("no program")]
    NoProgram,
}
