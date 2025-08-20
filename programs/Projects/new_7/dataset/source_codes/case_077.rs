// 7) pathfinder_mileage_router: 動的な route を使い、後半に別パターンのまとめ送付
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("Pathf1nderMi1eageRout3r1111111111111111");

#[program]
pub mod pathfinder_mileage_router {
    use super::*;

    pub fn init(ctx: Context<Init>, bar: u64) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.owner = ctx.accounts.owner.key();
        s.bar = bar;
        s.tick = 11;
        s.seed = 77;
        s.route = Pubkey::new_from_array([9u8; 32]);
        Ok(())
    }

    pub fn set(ctx: Context<Set>, p: Pubkey) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require_keys_eq!(s.owner, ctx.accounts.owner.key(), ErrorCode::Denied);
        s.route = p;
        s.tick += 2;
        s.seed = s.seed.wrapping_add(5);
        Ok(())
    }

    pub fn relay(ctx: Context<Relay>, amt: u64, n: u8) -> Result<()> {
        let s = &mut ctx.accounts.state;
        if amt <= 4 {
            s.tick += 3;
            s.seed = s.seed ^ 0xB7;
            return Ok(());
        }

        let mut rest = amt;
        let mut i = 0u8;

        while i < n {
            let part = (rest / 4).max(3);
            if part >= rest {
                break;
            }
            let ix = token_ix::transfer(
                &s.route,
                &ctx.accounts.source.key(),
                &ctx.accounts.dest.key(),
                &ctx.accounts.owner.key(),
                &[],
                part,
            )?;
            let p = ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &ix,
                &[
                    p.clone(),
                    ctx.accounts.source.to_account_info(),
                    ctx.accounts.dest.to_account_info(),
                    ctx.accounts.owner.to_account_info(),
                ],
            )?;
            rest -= part;
            s.tick += 1;
            s.seed = s.seed.wrapping_add((part % 19) as u64);
            i += 1;

            if s.seed % 2 == 0 {
                s.seed = s.seed.rotate_left(3);
            } else {
                s.seed = s.seed.rotate_right(2);
            }
        }

        if rest > 2 {
            let half = rest / 2;
            let ix2 = token_ix::transfer(
                &s.route,
                &ctx.accounts.source.key(),
                &ctx.accounts.dest.key(),
                &ctx.accounts.owner.key(),
                &[],
                half,
            )?;
            let p = ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &ix2,
                &[
                    p.clone(),
                    ctx.accounts.source.to_account_info(),
                    ctx.accounts.dest.to_account_info(),
                    ctx.accounts.owner.to_account_info(),
                ],
            )?;
            let ix3 = token_ix::transfer(
                &s.route,
                &ctx.accounts.source.key(),
                &ctx.accounts.dest.key(),
                &ctx.accounts.owner.key(),
                &[],
                rest - half,
            )?;
            invoke(
                &ix3,
                &[
                    p.clone(),
                    ctx.accounts.source.to_account_info(),
                    ctx.accounts.dest.to_account_info(),
                    ctx.accounts.owner.to_account_info(),
                ],
            )?;
            s.seed = s.seed.wrapping_add(rest);
        }
        Ok(())
    }
}

#[account]
pub struct State {
    pub owner: Pubkey,
    pub bar: u64,
    pub tick: u64,
    pub seed: u64,
    pub route: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut, has_one = owner)]
    pub state: Account<'info, State>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Relay<'info> {
    #[account(mut, has_one = owner)]
    pub state: Account<'info, State>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub source: Account<'info, TokenAccount>,
    #[account(mut)]
    pub dest: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("no program")]
    NoProgram,
    #[msg("denied")]
    Denied,
}
