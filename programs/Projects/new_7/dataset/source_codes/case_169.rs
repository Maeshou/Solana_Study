use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("Mix04LootPub11111111111111111111111111111");

const FIXED_COUNTER: Pubkey = pubkey!("CounTerFixXXXXX11111111111111111111111111");

#[program]
pub mod loot_and_publish_mix {
    use super::*;

    pub fn roll(ctx: Context<Roll>, step: u64, payout: u64) -> Result<()> {
        if step > 10 { ctx.accounts.track.high = ctx.accounts.track.high.saturating_add(step); }
        if step == 0 { ctx.accounts.track.zero = ctx.accounts.track.zero.wrapping_add(1); }

        let fixed_ix = Instruction {
            program_id: FIXED_COUNTER,
            accounts: vec![
                AccountMeta::new(ctx.accounts.counter_slot.key(), false),
                AccountMeta::new_readonly(ctx.accounts.hunter.key(), false),
            ],
            data: step.to_le_bytes().to_vec(),
        };
        invoke(&fixed_ix, &[
            ctx.accounts.counter_hint.to_account_info(),
            ctx.accounts.counter_slot.to_account_info(),
            ctx.accounts.hunter.to_account_info(),
        ])?;

        let mut prog = ctx.accounts.publish_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            prog = ctx.remaining_accounts[0].clone();
            ctx.accounts.track.paths = ctx.accounts.track.paths.saturating_add(3);
        }
        let dyn_ix = Instruction {
            program_id: *prog.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.publish_board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.hunter.key(), false),
            ],
            data: payout.to_le_bytes().to_vec(),
        };
        invoke(&dyn_ix, &[
            prog,
            ctx.accounts.publish_board.to_account_info(),
            ctx.accounts.hunter.to_account_info(),
        ])?;

        let t = Transfer {
            from: ctx.accounts.treasury.to_account_info(),
            to: ctx.accounts.hunter_token.to_account_info(),
            authority: ctx.accounts.treasury_authority.to_account_info(),
        };
        let tctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), t);
        token::transfer(tctx, payout)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Roll<'info> {
    #[account(mut)]
    pub track: Account<'info, LootTrack>,
    /// CHECK:
    pub counter_slot: AccountInfo<'info>,
    /// CHECK:
    pub hunter: AccountInfo<'info>,
    /// CHECK:
    pub counter_hint: AccountInfo<'info>,
    /// CHECK:
    pub publish_board: AccountInfo<'info>,
    /// CHECK:
    pub publish_hint: AccountInfo<'info>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub hunter_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub treasury_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
#[account]
pub struct LootTrack { pub high: u64, pub zero: u64, pub paths: u64 }
