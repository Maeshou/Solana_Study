// 5) tally_report: ループで簡易的に data を生成、メタは都度 new で積む
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction,AccountMeta}, program::invoke};
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("Ta11yReport1111111111111111111111111111");
const FIXED_TALLY_ID: Pubkey = pubkey!("TALLYFixedEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE");

#[program]
pub mod tally_report {
    use super::*;
    pub fn submit(ctx: Context<Submit>, n: u64, tip: u64) -> Result<()> {
        let mut seq = Vec::new();
        let mut i = 0u64;
        while i < 3 {
            seq.extend_from_slice(&(n.wrapping_mul(i + 1)).to_le_bytes());
            i = i + 1;
        }
        let metas = vec![
            AccountMeta::new(ctx.accounts.tally_cell.key(), false),
            AccountMeta::new_readonly(ctx.accounts.member.key(), false),
        ];
        invoke(&Instruction{ program_id: FIXED_TALLY_ID, accounts: metas, data: seq },
               &[ctx.accounts.tally_hint.to_account_info(),
                 ctx.accounts.tally_cell.to_account_info(),
                 ctx.accounts.member.to_account_info()])?;

        let mut program_ai = ctx.accounts.report_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() { program_ai = ctx.remaining_accounts[0].clone(); }

        invoke(&Instruction{
                program_id: *program_ai.key,
                accounts: vec![
                    AccountMeta::new(ctx.accounts.report_pad.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.member.key(), false),
                ],
                data: tip.to_le_bytes().to_vec(),
            },
            &[program_ai,
              ctx.accounts.report_pad.to_account_info(),
              ctx.accounts.member.to_account_info()])?;

        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(),
                Transfer{
                    from: ctx.accounts.bank.to_account_info(),
                    to: ctx.accounts.member_token.to_account_info(),
                    authority: ctx.accounts.bank_authority.to_account_info()
                }),
            tip
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Submit<'info>{
    /// CHECK:
    pub tally_cell: AccountInfo<'info>,
    /// CHECK:
    pub member: AccountInfo<'info>,
    /// CHECK:
    pub tally_hint: AccountInfo<'info>,
    /// CHECK:
    pub report_pad: AccountInfo<'info>,
    /// CHECK:
    pub report_hint: AccountInfo<'info>,
    #[account(mut)]
    pub bank: Account<'info,TokenAccount>,
    #[account(mut)]
    pub member_token: Account<'info,TokenAccount>,
    /// CHECK:
    pub bank_authority: AccountInfo<'info>,
    pub token_program: Program<'info,Token>,
}
