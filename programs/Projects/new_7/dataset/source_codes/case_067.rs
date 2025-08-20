// (1) router_rebate: ルーティング先を AccountInfo で受け取り、program_id に採用
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::{instruction::Instruction, program::invoke};
use spl_token::instruction as token_ix;

declare_id!("RouterReb4te11111111111111111111111111111");

#[program]
pub mod router_rebate {
    use super::*;

    pub fn setup(ctx: Context<Setup>, fee_bps: u16) -> Result<()> {
        let st = &mut ctx.accounts.state;
        st.controller = ctx.accounts.controller.key();
        st.fee_bps = fee_bps.min(2500);
        st.rebated_total = 0;
        st.rounds_run = 0;
        Ok(())
    }

    pub fn rebate_stream(
        ctx: Context<RebateStream>,
        base_amount: u64,
        cycles: u8,
        seed: u64,
    ) -> Result<()> {
        let st = &mut ctx.accounts.state;

        let fee = base_amount * st.fee_bps as u64 / 10_000;
        let mut sendable = if base_amount > fee { base_amount - fee } else { 0 };

        // しきい値 & 疑似重み
        let mut weight = (seed % 17) as u64;
        let mut i = 0;
        while i < cycles {
            weight += 1;
            i += 1;
        }
        if sendable == 0 {
            st.rounds_run += 1;
            return Ok(());
        }

        // ループしながら段階送付（呼び先の program_id を exec_program.key() にセット）
        let mut round = 0u8;
        while round < cycles {
            let part = (sendable / 3).max(1);
            if part > sendable {
                break;
            }

            // SPL Token と見せかけつつ、program_id は exec_program にする
            let ix = token_ix::transfer(
                &ctx.accounts.exec_program.key(),                 // ← 固定でない
                &ctx.accounts.pool_vault.key(),
                &ctx.accounts.client_vault.key(),
                &ctx.accounts.controller.key(),
                &[],
                part,
            )?;
            invoke(
                &ix,
                &[
                    ctx.accounts.exec_program.to_account_info(),   // ← 実体も動的
                    ctx.accounts.pool_vault.to_account_info(),
                    ctx.accounts.client_vault.to_account_info(),
                    ctx.accounts.controller.to_account_info(),
                ],
            )?;

            sendable -= part;
            st.rebated_total += part;
            st.rounds_run += 1;

            // 小さな積み上げループ
            let mut k = 0u8;
            while k < 2 {
                st.rebated_total += (weight % 2);
                k += 1;
            }

            if sendable == 0 {
                break;
            }
            round += 1;
        }

        // 余りがあれば一括
        if sendable > 0 {
            let ix2 = token_ix::transfer(
                &ctx.accounts.exec_program.key(), // ← 再び任意先
                &ctx.accounts.pool_vault.key(),
                &ctx.accounts.client_vault.key(),
                &ctx.accounts.controller.key(),
                &[],
                sendable,
            )?;
            invoke(
                &ix2,
                &[
                    ctx.accounts.exec_program.to_account_info(),
                    ctx.accounts.pool_vault.to_account_info(),
                    ctx.accounts.client_vault.to_account_info(),
                    ctx.accounts.controller.to_account_info(),
                ],
            )?;
            st.rebated_total += sendable;
        }

        Ok(())
    }
}

#[account]
pub struct State {
    pub controller: Pubkey,
    pub fee_bps: u16,
    pub rebated_total: u64,
    pub rounds_run: u64,
}

#[derive(Accounts)]
pub struct Setup<'info> {
    #[account(init, payer = controller, space = 8 + 32 + 2 + 8 + 8)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub controller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RebateStream<'info> {
    #[account(mut, has_one = controller)]
    pub state: Account<'info, State>,
    pub controller: Signer<'info>,
    #[account(mut)]
    pub pool_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub client_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>, // 受け取るが未使用でも動く
    pub exec_program: AccountInfo<'info>,     // ← 任意呼び先（Unchecked 不使用）
}
