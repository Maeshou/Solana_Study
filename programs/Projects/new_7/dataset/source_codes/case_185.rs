// 4) craft_grade_mix.rs
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, instruction::{Instruction, AccountMeta}};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("CrAfTGrAdEMiX11111111111111111111111111");

const FIXED_TALLY_ID: Pubkey = pubkey!("TaLLyFiXeD000000000000000000000000000000");

#[program]
pub mod craft_grade_mix {
    use super::*;

    impl<'info> DoCraft<'info> {
        fn SPL_send(&self, v: u64) -> Result<()> {
            token::transfer(
                CpiContext::new(
                    self.token_program.to_account_info(),
                    Transfer {
                        from: self.pool.to_account_info(),
                        to: self.user_token.to_account_info(),
                        authority: self.pool_authority.to_account_info(),
                    }
                ),
                v
            )
        }
    }

    pub fn exec(ctx: Context<DoCraft>, score: u64, gift: u64) -> Result<()> {
        if score > 80 {
            ctx.accounts.journal.gold = ctx.accounts.journal.gold.saturating_add(1);
        }

        let mut blob = score.to_le_bytes().to_vec();
        blob.extend_from_slice(&gift.to_le_bytes());

        // 固定ID
        let ix_fixed = Instruction {
            program_id: FIXED_TALLY_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.gauge_cell.key(), false),
                AccountMeta::new_readonly(ctx.accounts.student.key(), false),
            ],
            data: blob,
        };
        invoke(&ix_fixed, &[
            ctx.accounts.gauge_hint.to_account_info(),
            ctx.accounts.gauge_cell.to_account_info(),
            ctx.accounts.student.to_account_info(),
        ])?;

        // 動的CPI
        let mut prg = ctx.accounts.signal_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            prg = ctx.remaining_accounts[0].clone();
            ctx.accounts.journal.paths = ctx.accounts.journal.paths.saturating_add(3);
        }
        let ix_dyn = Instruction {
            program_id: *prg.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.signal_board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.student.key(), false),
            ],
            data: gift.rotate_left(6).to_le_bytes().to_vec(),
        };
        invoke(&ix_dyn, &[
            prg,
            ctx.accounts.signal_board.to_account_info(),
            ctx.accounts.student.to_account_info(),
        ])?;

        ctx.accounts.SPL_send(gift)
    }
}

#[derive(Accounts)]
pub struct DoCraft<'info> {
    #[account(mut)]
    pub journal: Account<'info, Journal>,
    /// CHECK:
    pub gauge_cell: AccountInfo<'info>,
    /// CHECK:
    pub student: AccountInfo<'info>,
    /// CHECK:
    pub gauge_hint: AccountInfo<'info>,
    /// CHECK:
    pub signal_board: AccountInfo<'info>,
    /// CHECK:
    pub signal_hint: AccountInfo<'info>,
    #[account(mut)]
    pub pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub pool_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
#[account]
pub struct Journal { pub gold: u64, pub paths: u64 }
