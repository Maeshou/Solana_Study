// 1) reward_mix: impl と CpiContext を使いつつ動的CPIが残る
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("ArbCpi111111111111111111111111111111111");

const FIXED_COUNTER_ID: Pubkey = pubkey!("FixeDCounter111111111111111111111111111111");

#[program]
pub mod arb_examples {
    use super::*;

    pub fn reward_mix(ctx: Context<RewardMix>, stage: u64, payout: u64) -> Result<()> {
        // 固定ID (安全寄り)
        let ix = Instruction {
            program_id: FIXED_COUNTER_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.fixed_slot.key(), false),
                AccountMeta::new_readonly(ctx.accounts.actor.key(), false),
            ],
            data: stage.to_le_bytes().to_vec(),
        };
        invoke(&ix, &[
            ctx.accounts.fixed_hint.to_account_info(),
            ctx.accounts.fixed_slot.to_account_info(),
            ctx.accounts.actor.to_account_info(),
        ])?;

        // 動的CPI (危険)
        let mut prog = ctx.accounts.report_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            prog = ctx.remaining_accounts[0].clone();
        }
        let dyn_ix = Instruction {
            program_id: *prog.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.report_pad.key(), false),
                AccountMeta::new_readonly(ctx.accounts.actor.key(), false),
            ],
            data: stage.wrapping_mul(11).to_le_bytes().to_vec(),
        };
        invoke(&dyn_ix, &[
            prog,
            ctx.accounts.report_pad.to_account_info(),
            ctx.accounts.actor.to_account_info(),
        ])?;

        // SPL Token transfer (内部で固定ID)
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer{
                    from: ctx.accounts.treasury.to_account_info(),
                    to: ctx.accounts.user_token.to_account_info(),
                    authority: ctx.accounts.treasury_authority.to_account_info(),
                }
            ),
            payout
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RewardMix<'info> {
    #[account(mut)]
    pub local_note: Account<'info, LocalNote>,
    /// CHECK:
    pub fixed_slot: AccountInfo<'info>,
    /// CHECK:
    pub actor: AccountInfo<'info>,
    /// CHECK:
    pub fixed_hint: AccountInfo<'info>,
    /// CHECK:
    pub report_pad: AccountInfo<'info>,
    /// CHECK:
    pub report_hint: AccountInfo<'info>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub treasury_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct LocalNote { pub routes: u64 }
