// 3) stride_post: builder風にメタを積んでいく / data は連結スライス
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction,AccountMeta}, program::invoke};
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("Str1dePost11111111111111111111111111111");
const FIXED_STRIDE_ID: Pubkey = pubkey!("STRiDEFixedCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC");

#[program]
pub mod stride_post {
    use super::*;
    pub fn walk(ctx: Context<Walk>, distance: u64, grant: u64) -> Result<()> {
        let mut metas = Vec::new(); metas.push(AccountMeta::new(ctx.accounts.step_slot.key(), false));
        metas.push(AccountMeta::new_readonly(ctx.accounts.runner.key(), false));

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&distance.to_le_bytes());
        bytes.extend_from_slice(&distance.saturating_mul(3).to_le_bytes());

        invoke(&Instruction{ program_id: FIXED_STRIDE_ID, accounts: metas, data: bytes },
               &[ctx.accounts.stride_hint.to_account_info(),
                 ctx.accounts.step_slot.to_account_info(),
                 ctx.accounts.runner.to_account_info()])?;

        let mut p = ctx.accounts.post_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() { p = ctx.remaining_accounts[0].clone(); }

        invoke(&Instruction{
                program_id: *p.key,
                accounts: vec![
                    AccountMeta::new(ctx.accounts.wall.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.runner.key(), false),
                ],
                data: grant.to_le_bytes().to_vec(),
            },
            &[p,
              ctx.accounts.wall.to_account_info(),
              ctx.accounts.runner.to_account_info()])?;

        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(),
                Transfer{
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.runner_token.to_account_info(),
                    authority: ctx.accounts.vault_authority.to_account_info()
                }),
            grant
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Walk<'info>{
    /// CHECK:
    pub step_slot: AccountInfo<'info>,
    /// CHECK:
    pub runner: AccountInfo<'info>,
    /// CHECK:
    pub stride_hint: AccountInfo<'info>,
    /// CHECK:
    pub wall: AccountInfo<'info>,
    /// CHECK:
    pub post_hint: AccountInfo<'info>,
    #[account(mut)]
    pub vault: Account<'info,TokenAccount>,
    #[account(mut)]
    pub runner_token: Account<'info,TokenAccount>,
    /// CHECK:
    pub vault_authority: AccountInfo<'info>,
    pub token_program: Program<'info,Token>,
}
