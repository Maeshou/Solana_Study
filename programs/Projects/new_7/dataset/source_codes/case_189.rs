// B) ヘルパ関数で AccountMeta を段階的に組み立て（push で可読性を変える）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("MiXAltB111111111111111111111111111111111");

const FIXED_ID: Pubkey = pubkey!("FiXeDBBBBB000000000000000000000000000000");

#[program]
pub mod mix_alt_b {
    use super::*;

    fn build_two_metas(main: Pubkey, actor: Pubkey) -> Vec<AccountMeta> {
        let mut metas = Vec::with_capacity(2);
        metas.push(AccountMeta::new(main, false));
        metas.push(AccountMeta::new_readonly(actor, false));
        metas
    }

    pub fn run(ctx: Context<Run>, tag: u64, pay: u64) -> Result<()> {
        // 固定ID: メタをヘルパで構築
        let metas = build_two_metas(ctx.accounts.cell.key(), ctx.accounts.user.key());
        invoke(
            &Instruction { program_id: FIXED_ID, accounts: metas, data: tag.to_le_bytes().to_vec() },
            &[
                ctx.accounts.hint.to_account_info(),
                ctx.accounts.cell.to_account_info(),
                ctx.accounts.user.to_account_info(),
            ],
        )?;

        // 動的CPI: メタは別ヘルパ再利用、program_id は AccountInfo から採用
        let mut p = ctx.accounts.router_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            p = ctx.remaining_accounts[0].clone();
            ctx.accounts.stat.count = ctx.accounts.stat.count.saturating_add(2);
        }
        let metas2 = build_two_metas(ctx.accounts.router_board.key(), ctx.accounts.user.key());
        invoke(
            &Instruction { program_id: *p.key, accounts: metas2, data: pay.rotate_right(3).to_le_bytes().to_vec() },
            &[
                p,
                ctx.accounts.router_board.to_account_info(),
                ctx.accounts.user.to_account_info(),
            ],
        )?;

        // SPL
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.bank.to_account_info(),
                    to: ctx.accounts.user_token.to_account_info(),
                    authority: ctx.accounts.bank_auth.to_account_info(),
                },
            ),
            pay,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Run<'info> {
    #[account(mut)] pub stat: Account<'info, Stat>,
    /// CHECK: 
    pub cell: AccountInfo<'info>,
    /// CHECK:
    pub hint: AccountInfo<'info>,
    /// CHECK:
    pub user: AccountInfo<'info>,
    /// CHECK:
    pub router_board: AccountInfo<'info>,
    /// CHECK:
    pub router_hint: AccountInfo<'info>,
    #[account(mut)] pub bank: Account<'info, TokenAccount>,
    #[account(mut)] pub user_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub bank_auth: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
#[account] pub struct Stat { pub count: u64 }
