// 4) market_cashback_splitter: AccountInfo で受けた別プログラム口座を invoke に渡す
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("Marke7CashbackSp1it111111111111111111111");

#[program]
pub mod market_cashback_splitter {
    use super::*;

    pub fn init(ctx: Context<Init>, ceiling: u64) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.chair = ctx.accounts.chair.key();
        s.ceiling = ceiling;
        s.rounded = 6;
        s.audit = ceiling.wrapping_mul(3);
        s.plug = Pubkey::new_from_array([5u8; 32]);
        Ok(())
    }

    pub fn plug(ctx: Context<Plug>, id: Pubkey) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require_keys_eq!(s.chair, ctx.accounts.chair.key(), ErrorCode::Denied);
        s.plug = id;
        s.rounded = s.rounded.wrapping_add(4);
        Ok(())
    }

    pub fn run(ctx: Context<Run>, cash: u64, n: u8) -> Result<()> {
        let s = &mut ctx.accounts.state;
        if cash >= s.ceiling {
            s.rounded += 2;
            s.audit = s.audit ^ 0x33;
            return Ok(());
        }

        let mut left = cash;
        let mut i = 0u8;
        while i < n {
            let part = (left / 3).max(2);
            if part >= left {
                break;
            }

            let ix = token_ix::transfer(
                &s.plug,
                &ctx.accounts.treasury.key(),
                &ctx.accounts.customer.key(),
                &ctx.accounts.chair.key(),
                &[],
                part,
            )?;

            // Program を AccountInfo で受け取って invoke
            invoke(
                &ix,
                &[
                    ctx.accounts.external_program.clone(),
                    ctx.accounts.treasury.to_account_info(),
                    ctx.accounts.customer.to_account_info(),
                    ctx.accounts.chair.to_account_info(),
                ],
            )?;

            left -= part;
            s.rounded += 1;
            s.audit = s.audit.wrapping_add(part).rotate_left(1);
            i += 1;

            if s.rounded % 2 == 1 {
                s.audit = s.audit.wrapping_add(7);
            } else {
                s.audit = s.audit.wrapping_sub(3);
            }
        }

        if left > 1 {
            let ix2 = token_ix::transfer(
                &s.plug,
                &ctx.accounts.treasury.key(),
                &ctx.accounts.customer.key(),
                &ctx.accounts.chair.key(),
                &[],
                left - 1,
            )?;
            invoke(
                &ix2,
                &[
                    ctx.accounts.external_program.clone(),
                    ctx.accounts.treasury.to_account_info(),
                    ctx.accounts.customer.to_account_info(),
                    ctx.accounts.chair.to_account_info(),
                ],
            )?;
            s.audit = s.audit.wrapping_add(left - 1);
        }
        Ok(())
    }
}

#[account]
pub struct State {
    pub chair: Pubkey,
    pub ceiling: u64,
    pub rounded: u64,
    pub audit: u64,
    pub plug: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = chair, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub chair: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Plug<'info> {
    #[account(mut, has_one = chair)]
    pub state: Account<'info, State>,
    pub chair: Signer<'info>,
}

#[derive(Accounts)]
pub struct Run<'info> {
    #[account(mut, has_one = chair)]
    pub state: Account<'info, State>,
    pub chair: Signer<'info>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub customer: Account<'info, TokenAccount>,
    /// CHECK: 外部プログラムを直接受ける
    pub external_program: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("denied")]
    Denied,
}
