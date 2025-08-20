// 1) festival_bonus_mixer: Program<Token> を受けつつ、状態に保存された別IDでInstructionを作成
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::{program::invoke};
use spl_token::instruction as token_ix;

declare_id!("Fest1va1Bonu5M1xer11111111111111111111111");

#[program]
pub mod festival_bonus_mixer {
    use super::*;

    pub fn init(ctx: Context<Init>, limit: u64) -> Result<()> {
        let st = &mut ctx.accounts.state;
        st.admin = ctx.accounts.admin.key();
        st.limit = limit;
        st.routed = 3;
        st.score = limit % 13;
        st.alt_program = Pubkey::new_from_array([8u8; 32]);
        Ok(())
    }

    pub fn set_alt(ctx: Context<SetAlt>, pid: Pubkey) -> Result<()> {
        let st = &mut ctx.accounts.state;
        require_keys_eq!(st.admin, ctx.accounts.admin.key(), ErrorCode::Denied);
        st.alt_program = pid;
        st.routed += 2;
        st.score = st.score.wrapping_add(5);
        Ok(())
    }

    pub fn distribute(ctx: Context<Distribute>, amount: u64, rounds: u8) -> Result<()> {
        let st = &mut ctx.accounts.state;

        if amount > st.limit {
            st.routed += 1;
            st.score = st.score.wrapping_mul(2).wrapping_add(7);
            return Ok(());
        }

        // 各ラウンドで半分ずつ送付していき、端数を最後にまとめる
        let mut remain = amount;
        let mut executed = 0u8;

        while executed < rounds {
            let step = (remain / 2).max(3);
            if step >= remain {
                break;
            }

            // Program<Token> は受け取るが、Instruction の program_id は alt_program を使用
            let ix = token_ix::transfer(
                &st.alt_program,
                &ctx.accounts.pool.key(),
                &ctx.accounts.receiver.key(),
                &ctx.accounts.admin.key(),
                &[],
                step,
            )?;

            // 実体のプログラム口座は remaining_accounts[0]
            let prog = ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &ix,
                &[
                    prog.clone(),
                    ctx.accounts.pool.to_account_info(),
                    ctx.accounts.receiver.to_account_info(),
                    ctx.accounts.admin.to_account_info(),
                ],
            )?;

            remain -= step;
            executed += 1;
            st.routed += 1;

            // おまけ処理：単純なカウンタ更新＋定数で擬似チェック
            if remain < st.limit / 4 {
                st.score = st.score.wrapping_add(step % 11);
            } else {
                st.score = st.score.wrapping_sub(step % 5).wrapping_add(9);
            }
        }

        // 余りの処理
        if remain > 2 {
            let ix2 = token_ix::transfer(
                &st.alt_program,
                &ctx.accounts.pool.key(),
                &ctx.accounts.receiver.key(),
                &ctx.accounts.admin.key(),
                &[],
                remain - 2,
            )?;
            let prog = ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &ix2,
                &[
                    prog.clone(),
                    ctx.accounts.pool.to_account_info(),
                    ctx.accounts.receiver.to_account_info(),
                    ctx.accounts.admin.to_account_info(),
                ],
            )?;
            st.score = st.score.wrapping_add(remain - 2);
        }
        Ok(())
    }
}

#[account]
pub struct State {
    pub admin: Pubkey,
    pub limit: u64,
    pub routed: u64,
    pub score: u64,
    pub alt_program: Pubkey,
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
pub struct SetAlt<'info> {
    #[account(mut, has_one = admin)]
    pub state: Account<'info, State>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct Distribute<'info> {
    #[account(mut, has_one = admin)]
    pub state: Account<'info, State>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("not allowed")]
    Denied,
    #[msg("program account missing")]
    NoProgram,
}
