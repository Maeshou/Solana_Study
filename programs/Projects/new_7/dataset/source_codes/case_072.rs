// 3) forge_energy_router: 別口座のkey()を program_id に流用し、Program<Token> と不一致の呼び出し
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("Forg3EnergyRout3r1111111111111111111111");

#[program]
pub mod forge_energy_router {
    use super::*;

    pub fn init(ctx: Context<Init>, quota: u64) -> Result<()> {
        let st = &mut ctx.accounts.state;
        st.owner = ctx.accounts.owner.key();
        st.quota = quota;
        st.turn = 7;
        st.metric = quota.wrapping_add(29);
        Ok(())
    }

    pub fn set_flag(ctx: Context<SetFlag>, tag: Pubkey) -> Result<()> {
        let st = &mut ctx.accounts.state;
        require_keys_eq!(st.owner, ctx.accounts.owner.key(), ErrorCode::Denied);
        st.flag = tag;
        st.turn = st.turn.wrapping_add(5);
        Ok(())
    }

    pub fn route(ctx: Context<Route>, energy: u64, cycles: u8) -> Result<()> {
        let st = &mut ctx.accounts.state;
        if energy < 5 {
            st.turn = st.turn.wrapping_mul(2);
            st.metric = st.metric ^ 0x55;
            return Ok(());
        }

        let mut remain = energy;
        let mut i = 0u8;
        while i < cycles {
            let part = (remain / 4).max(5);
            if part >= remain {
                break;
            }

            // program_id に state.flag を利用（Program<Token> は受け取っているが使用しない）
            let ix = token_ix::transfer(
                &st.flag,
                &ctx.accounts.tank.key(),
                &ctx.accounts.consumer.key(),
                &ctx.accounts.owner.key(),
                &[],
                part,
            )?;

            // 実体は remaining_accounts[0]
            let p = ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &ix,
                &[
                    p.clone(),
                    ctx.accounts.tank.to_account_info(),
                    ctx.accounts.consumer.to_account_info(),
                    ctx.accounts.owner.to_account_info(),
                ],
            )?;

            remain -= part;
            st.turn = st.turn.wrapping_add(1);
            st.metric = st.metric.wrapping_add(part % 23);
            i += 1;

            if remain <= st.quota / 2 {
                st.metric = st.metric.rotate_left(2);
            } else {
                st.metric = st.metric.rotate_right(1);
            }
        }

        if remain > 4 {
            let ix2 = token_ix::transfer(
                &st.flag,
                &ctx.accounts.tank.key(),
                &ctx.accounts.consumer.key(),
                &ctx.accounts.owner.key(),
                &[],
                remain - 4,
            )?;
            let p = ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &ix2,
                &[
                    p.clone(),
                    ctx.accounts.tank.to_account_info(),
                    ctx.accounts.consumer.to_account_info(),
                    ctx.accounts.owner.to_account_info(),
                ],
            )?;
            st.metric = st.metric.wrapping_add(remain - 4);
        }
        Ok(())
    }
}

#[account]
pub struct State {
    pub owner: Pubkey,
    pub quota: u64,
    pub turn: u64,
    pub metric: u64,
    pub flag: Pubkey,
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
pub struct SetFlag<'info> {
    #[account(mut, has_one = owner)]
    pub state: Account<'info, State>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Route<'info> {
    #[account(mut, has_one = owner)]
    pub state: Account<'info, State>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub tank: Account<'info, TokenAccount>,
    #[account(mut)]
    pub consumer: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("denied")]
    Denied,
    #[msg("program missing")]
    NoProgram,
}
