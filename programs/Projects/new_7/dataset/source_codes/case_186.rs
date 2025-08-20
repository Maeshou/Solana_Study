// 5) hatch_route_mix.rs
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("HaTcHRoUtEMiX11111111111111111111111111");

const FIXED_BOOK_ID: Pubkey = pubkey!("BoOkFiXeD0000000000000000000000000000000");

#[program]
pub mod hatch_route_mix {
    use super::*;

    fn mint_bonus(tp: &Program<Token>, chest: &Account<TokenAccount>, owner: &Account<TokenAccount>, auth: &AccountInfo, v: u64) -> Result<()> {
        token::transfer(
            CpiContext::new(tp.to_account_info(), Transfer {
                from: chest.to_account_info(), to: owner.to_account_info(), authority: auth.clone()
            }),
            v
        )
    }

    pub fn hatch(ctx: Context<Hatch>, seed: u64, grant: u64) -> Result<()> {
        if seed > 500 {
            ctx.accounts.note.count = ctx.accounts.note.count.saturating_add(1);
        }

        // 固定ID
        let ix_fixed = Instruction {
            program_id: FIXED_BOOK_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.hatch_cell.key(), false),
                AccountMeta::new_readonly(ctx.accounts.owner.key(), false),
            ],
            data: seed.to_le_bytes().to_vec(),
        };
        invoke(&ix_fixed, &[
            ctx.accounts.hatch_hint.to_account_info(),
            ctx.accounts.hatch_cell.to_account_info(),
            ctx.accounts.owner.to_account_info(),
        ])?;

        // 動的CPI
        let mut nprg = ctx.accounts.notice_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            nprg = ctx.remaining_accounts[0].clone();
        }
        let ix_dyn = Instruction {
            program_id: *nprg.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.notice_board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.owner.key(), false),
            ],
            data: grant.rotate_right(7).to_le_bytes().to_vec(),
        };
        invoke(&ix_dyn, &[
            nprg,
            ctx.accounts.notice_board.to_account_info(),
            ctx.accounts.owner.to_account_info(),
        ])?;

        mint_bonus(&ctx.accounts.token_program, &ctx.accounts.chest, &ctx.accounts.owner_token, &ctx.accounts.chest_authority, grant)
    }
}

#[derive(Accounts)]
pub struct Hatch<'info> {
    #[account(mut)]
    pub note: Account<'info, HatchNote>,
    /// CHECK:
    pub hatch_cell: AccountInfo<'info>,
    /// CHECK:
    pub owner: AccountInfo<'info>,
    /// CHECK:
    pub hatch_hint: AccountInfo<'info>,
    /// CHECK:
    pub notice_board: AccountInfo<'info>,
    /// CHECK:
    pub notice_hint: AccountInfo<'info>,
    #[account(mut)]
    pub chest: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub chest_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
#[account]
pub struct HatchNote { pub count: u64 }
