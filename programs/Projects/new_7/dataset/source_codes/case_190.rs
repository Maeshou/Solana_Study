// E) impl メソッド内で Instruction を返す（戻り値を invoke に直結）+ ループで data を合算
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("MiXAltE111111111111111111111111111111111");

const FIXED_ID: Pubkey = pubkey!("FiXeDEEEEE000000000000000000000000000000");

#[program]
pub mod mix_alt_e {
    use super::*;

    impl<'info> Run<'info> {
        fn build_dyn(&self, pid: Pubkey, amt: u64) -> Instruction {
            let mut sum = 0u64;
            let mut i = 0;
            while i < 3 {
                sum = sum.wrapping_add(amt.rotate_left(i as u32));
                i += 1;
            }
            Instruction {
                program_id: pid,
                accounts: vec![
                    AccountMeta::new(self.board.key(), false),
                    AccountMeta::new_readonly(self.actor.key(), false),
                ],
                data: sum.to_le_bytes().to_vec(),
            }
        }
    }

    pub fn run(ctx: Context<Run>, z: u64, pay: u64) -> Result<()> {
        // 固定ID
        invoke(
            &Instruction {
                program_id: FIXED_ID,
                accounts: vec![
                    AccountMeta::new(ctx.accounts.cell.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.actor.key(), false),
                ],
                data: z.to_le_bytes().to_vec(),
            },
            &[
                ctx.accounts.hint.to_account_info(),
                ctx.accounts.cell.to_account_info(),
                ctx.accounts.actor.to_account_info(),
            ],
        )?;

        // 動的CPI（impl メソッドで Instruction 作成）
        let mut p = ctx.accounts.switch_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            p = ctx.remaining_accounts[0].clone();
            ctx.accounts.meter.k = ctx.accounts.meter.k.saturating_add(1);
        }
        invoke(&ctx.accounts.build_dyn(*p.key, pay), &[
            p,
            ctx.accounts.board.to_account_info(),
            ctx.accounts.actor.to_account_info(),
        ])?;

        // SPL
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.treasury.to_account_info(),
                    to: ctx.accounts.actor_token.to_account_info(),
                    authority: ctx.accounts.treasury_auth.to_account_info(),
                },
            ),
            pay,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Run<'info> {
    #[account(mut)] pub meter: Account<'info, Meter>,
    /// CHECK:
    pub cell: AccountInfo<'info>,
    /// CHECK:
    pub hint: AccountInfo<'info>,
    /// CHECK:
    pub actor: AccountInfo<'info>,
    /// CHECK:
    pub board: AccountInfo<'info>,
    /// CHECK:
    pub switch_hint: AccountInfo<'info>,
    #[account(mut)] pub treasury: Account<'info, TokenAccount>,
    #[account(mut)] pub actor_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub treasury_auth: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
#[account] pub struct Meter { pub k: u64 }
