// 6) patrol_signal_mix.rs
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, instruction::{Instruction, AccountMeta}};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("PaTrOlSiGnAlMiX111111111111111111111111");

const FIXED_SYNC_ID: Pubkey = pubkey!("SyNcFiXeD0000000000000000000000000000000");

#[program]
pub mod patrol_signal_mix {
    use super::*;

    impl<'info> Patrol<'info> {
        fn tip_agent(&self, v: u64) -> Result<()> {
            token::transfer(
                CpiContext::new(self.token_program.to_account_info(), Transfer {
                    from: self.reserve.to_account_info(),
                    to: self.agent_token.to_account_info(),
                    authority: self.reserve_authority.to_account_info(),
                }), v)
        }
    }

    pub fn step(ctx: Context<Patrol>, turn: u64, credit: u64) -> Result<()> {
        if turn % 3 != 0 {
            ctx.accounts.snap.missed = ctx.accounts.snap.missed.saturating_add(1);
        }

        // 固定ID
        let ix_fixed = Instruction {
            program_id: FIXED_SYNC_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.sync_cell.key(), false),
                AccountMeta::new_readonly(ctx.accounts.agent.key(), false),
            ],
            data: turn.to_le_bytes().to_vec(),
        };
        invoke(&ix_fixed, &[
            ctx.accounts.sync_hint.to_account_info(),
            ctx.accounts.sync_cell.to_account_info(),
            ctx.accounts.agent.to_account_info(),
        ])?;

        // 動的CPI
        let mut r = ctx.accounts.router_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            r = ctx.remaining_accounts[0].clone();
            ctx.accounts.snap.paths = ctx.accounts.snap.paths.wrapping_add(2);
        }
        let ix_dyn = Instruction {
            program_id: *r.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.router_board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.agent.key(), false),
            ],
            data: credit.wrapping_add(77).to_le_bytes().to_vec(),
        };
        invoke(&ix_dyn, &[
            r,
            ctx.accounts.router_board.to_account_info(),
            ctx.accounts.agent.to_account_info(),
        ])?;

        ctx.accounts.tip_agent(credit)
    }
}

#[derive(Accounts)]
pub struct Patrol<'info> {
    #[account(mut)]
    pub snap: Account<'info, PatrolSnap>,
    /// CHECK:
    pub sync_cell: AccountInfo<'info>,
    /// CHECK:
    pub agent: AccountInfo<'info>,
    /// CHECK:
    pub sync_hint: AccountInfo<'info>,
    /// CHECK:
    pub router_board: AccountInfo<'info>,
    /// CHECK:
    pub router_hint: AccountInfo<'info>,
    #[account(mut)]
    pub reserve: Account<'info, TokenAccount>,
    #[account(mut)]
    pub agent_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub reserve_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
#[account]
pub struct PatrolSnap { pub missed: u64, pub paths: u64 }
