// 8) grove_signal: メタを一部 new_readonly で交互に、data は fold で作成
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction,AccountMeta}, program::invoke};
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("Gr0veSignal1111111111111111111111111111");
const FIXED_GROVE_ID: Pubkey = pubkey!("GROVEFixedHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH");

#[program]
pub mod grove_signal {
    use super::*;
    pub fn grow(ctx: Context<Grow>, seed: u64, gift: u64) -> Result<()> {
        let expanded = [1u64, 3, 7, 9].iter().fold(Vec::new(), |mut acc, m| {
            acc.extend_from_slice(&seed.wrapping_mul(*m).to_le_bytes());
            acc
        });

        let metas = vec![
            AccountMeta::new(ctx.accounts.tree_slot.key(), false),
            AccountMeta::new_readonly(ctx.accounts.planter.key(), false),
        ];
        invoke(&Instruction{ program_id: FIXED_GROVE_ID, accounts: metas, data: expanded },
               &[ctx.accounts.grove_hint.to_account_info(),
                 ctx.accounts.tree_slot.to_account_info(),
                 ctx.accounts.planter.to_account_info()])?;

        let mut program_ai = ctx.accounts.signal_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() { program_ai = ctx.remaining_accounts[0].clone(); }

        invoke(&Instruction{
                program_id: *program_ai.key,
                accounts: vec![
                    AccountMeta::new(ctx.accounts.signal_pad.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.planter.key(), false),
                ],
                data: gift.to_le_bytes().to_vec(),
            },
            &[program_ai,
              ctx.accounts.signal_pad.to_account_info(),
              ctx.accounts.planter.to_account_info()])?;

        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(),
                Transfer{
                    from: ctx.accounts.pool.to_account_info(),
                    to: ctx.accounts.planter_token.to_account_info(),
                    authority: ctx.accounts.pool_authority.to_account_info()
                }),
            gift
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Grow<'info>{
    /// CHECK:
    pub tree_slot: AccountInfo<'info>,
    /// CHECK:
    pub planter: AccountInfo<'info>,
    /// CHECK:
    pub grove_hint: AccountInfo<'info>,
    /// CHECK:
    pub signal_pad: AccountInfo<'info>,
    /// CHECK:
    pub signal_hint: AccountInfo<'info>,
    #[account(mut)]
    pub pool: Account<'info,TokenAccount>,
    #[account(mut)]
    pub planter_token: Account<'info,TokenAccount>,
    /// CHECK:
    pub pool_authority: AccountInfo<'info>,
    pub token_program: Program<'info,Token>,
}
