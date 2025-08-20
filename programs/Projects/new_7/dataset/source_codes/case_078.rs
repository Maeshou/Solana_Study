// 8) caravan_loyalty_engine: Program<Token> を無視して alt を使う＋二重のまとめ処理
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("CaravanL0ya1tyEng1ne11111111111111111111");

#[program]
pub mod caravan_loyalty_engine {
    use super::*;

    pub fn init(ctx: Context<Init>, bar: u64) -> Result<()> {
        let st = &mut ctx.accounts.state;
        st.admin = ctx.accounts.admin.key();
        st.bar = bar;
        st.count = 12;
        st.hash = bar.rotate_left(2);
        st.alt = Pubkey::new_from_array([4u8; 32]);
        Ok(())
    }

    pub fn bind(ctx: Context<Bind>, id: Pubkey) -> Result<()> {
        let st = &mut ctx.accounts.state;
        require_keys_eq!(st.admin, ctx.accounts.admin.key(), ErrorCode::Denied);
        st.alt = id;
        st.count = st.count.wrapping_add(2);
        Ok(())
    }

    pub fn process(ctx: Context<Process>, n: u64, r: u8) -> Result<()> {
        let st = &mut ctx.accounts.state;

        if n < 3 {
            st.hash = st.hash ^ 0xFA;
            st.count += 1;
            return Ok(());
        }

        let mut left = n;
        let mut i = 0u8;
        while i < r {
            let part = (left / 3).max(2);
            if part >= left {
                break;
            }
            let ix = token_ix::transfer(
                &st.alt,
                &ctx.accounts.vault.key(),
                &ctx.accounts.user.key(),
                &ctx.accounts.admin.key(),
                &[],
                part,
            )?;
            let p = ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &ix,
                &[
                    p.clone(),
                    ctx.accounts.vault.to_account_info(),
                    ctx.accounts.user.to_account_info(),
                    ctx.accounts.admin.to_account_info(),
                ],
            )?;

            left -= part;
            st.count += 1;
            st.hash = st.hash.wrapping_add(part).rotate_right(2);
            i += 1;

            if st.count % 3 == 1 {
                st.hash = st.hash.wrapping_add(15);
            } else {
                st.hash = st.hash.wrapping_sub(6).wrapping_add(2);
            }
        }

        if left > 2 {
            let half = left / 2;
            let ix2 = token_ix::transfer(
                &st.alt,
                &ctx.accounts.vault.key(),
                &ctx.accounts.user.key(),
                &ctx.accounts.admin.key(),
                &[],
                half,
            )?;
            let p = ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &ix2,
                &[
                    p.clone(),
                    ctx.accounts.vault.to_account_info(),
                    ctx.accounts.user.to_account_info(),
                    ctx.accounts.admin.to_account_info(),
                ],
            )?;
            let ix3 = token_ix::transfer(
                &st.alt,
                &ctx.accounts.vault.key(),
                &ctx.accounts.user.key(),
                &ctx.accounts.admin.key(),
                &[],
                left - half,
            )?;
            invoke(
                &ix3,
                &[
                    p.clone(),
                    ctx.accounts.vault.to_account_info(),
                    ctx.accounts.user.to_account_info(),
                    ctx.accounts.admin.to_account_info(),
                ],
            )?;
            st.hash = st.hash.wrapping_add(left);
        }
        Ok(())
    }
}

#[account]
pub struct State {
    pub admin: Pubkey,
    pub bar: u64,
    pub count: u64,
    pub hash: u64,
    pub alt: Pubkey,
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
pub struct Bind<'info> {
    #[account(mut, has_one = admin)]
    pub state: Account<'info, State>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct Process<'info> {
    #[account(mut, has_one = admin)]
    pub state: Account<'info, State>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("no program")]
    NoProgram,
    #[msg("denied")]
    Denied,
}
