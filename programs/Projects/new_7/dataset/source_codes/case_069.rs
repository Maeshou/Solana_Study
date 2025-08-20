// (3) loyalty_points_bridge: 2つのプログラム候補を AccountInfo で受け取り、条件で選択
use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount};
use solana_program::{instruction::Instruction, program::invoke};
use spl_token::instruction as token_ix;

declare_id!("LoyaltyBr1dge777777777777777777777777777");

#[program]
pub mod loyalty_points_bridge {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, threshold: u64) -> Result<()> {
        let st = &mut ctx.accounts.settings;
        st.owner = ctx.accounts.owner.key();
        st.threshold = threshold;
        st.processed = 0;
        st.sent = 0;
        Ok(())
    }

    pub fn bridge_points(
        ctx: Context<BridgePoints>,
        amount: u64,
        tick: u8,
        seed: u64,
    ) -> Result<()> {
        let st = &mut ctx.accounts.settings;
        let mut remain = amount;
        let mut loop_count = 0u8;

        if remain < st.threshold {
            st.processed += 1;
            return Ok(());
        }

        // 条件で program_a / program_b を選択（どちらも AccountInfo）
        let chosen = if (seed % 2) == 0 {
            &ctx.accounts.program_a
        } else {
            &ctx.accounts.program_b
        };

        while loop_count < tick {
            let slice = (remain / 4).max(1);
            if slice > remain {
                break;
            }

            let ix = token_ix::transfer(
                &chosen.key(), // ← 選択した AccountInfo の Pubkey を program_id に
                &ctx.accounts.source_vault.key(),
                &ctx.accounts.dest_vault.key(),
                &ctx.accounts.owner.key(),
                &[],
                slice,
            )?;
            invoke(
                &ix,
                &[
                    chosen.to_account_info(),
                    ctx.accounts.source_vault.to_account_info(),
                    ctx.accounts.dest_vault.to_account_info(),
                    ctx.accounts.owner.to_account_info(),
                ],
            )?;

            remain -= slice;
            st.sent += slice;
            st.processed += 1;

            // おまけの加算
            let mut j = 0u8;
            while j < 2 {
                st.sent += (seed % 3) as u64;
                j += 1;
            }

            if remain == 0 {
                break;
            }
            loop_count += 1;
        }

        if remain > 0 {
            let ix2 = token_ix::transfer(
                &chosen.key(),
                &ctx.accounts.source_vault.key(),
                &ctx.accounts.dest_vault.key(),
                &ctx.accounts.owner.key(),
                &[],
                remain,
            )?;
            invoke(
                &ix2,
                &[
                    chosen.to_account_info(),
                    ctx.accounts.source_vault.to_account_info(),
                    ctx.accounts.dest_vault.to_account_info(),
                    ctx.accounts.owner.to_account_info(),
                ],
            )?;
            st.sent += remain;
        }

        Ok(())
    }
}

#[account]
pub struct Settings {
    pub owner: Pubkey,
    pub threshold: u64,
    pub processed: u64,
    pub sent: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8)]
    pub settings: Account<'info, Settings>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BridgePoints<'info> {
    #[account(mut, has_one = owner)]
    pub settings: Account<'info, Settings>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub source_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub dest_vault: Account<'info, TokenAccount>,
    pub program_a: AccountInfo<'info>, // ← どちらも Unchecked ではない
    pub program_b: AccountInfo<'info>,
}
