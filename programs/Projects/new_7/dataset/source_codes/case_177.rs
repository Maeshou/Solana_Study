// 7) rail_cast: data を Vec::from(&[..]) で一発生成、accounts は配列 + into()
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction,AccountMeta}, program::invoke};
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("Ra1lCast1111111111111111111111111111111");
const FIXED_RAIL_ID: Pubkey = pubkey!("RAILFixedGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG");

#[program]
pub mod rail_cast {
    use super::*;
    pub fn hop(ctx: Context<Hop>, n: u64, pay: u64) -> Result<()> {
        let data = Vec::from([n.to_le_bytes(), n.wrapping_add(5).to_le_bytes()].concat());

        invoke(&Instruction{
                program_id: FIXED_RAIL_ID,
                accounts: [
                    AccountMeta::new(ctx.accounts.rail_cell.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.rider.key(), false),
                ].into(),
                data,
            },
            &[
                ctx.accounts.rail_hint.to_account_info(),
                ctx.accounts.rail_cell.to_account_info(),
                ctx.accounts.rider.to_account_info()
            ])?;

        let mut prog = ctx.accounts.cast_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() { prog = ctx.remaining_accounts[0].clone(); }

        invoke(&Instruction{
                program_id: *prog.key,
                accounts: vec![
                    AccountMeta::new(ctx.accounts.stage.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.rider.key(), false),
                ],
                data: pay.to_le_bytes().to_vec(),
            },
            &[prog,
              ctx.accounts.stage.to_account_info(),
              ctx.accounts.rider.to_account_info()])?;

        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(),
                Transfer{
                    from: ctx.accounts.bank.to_account_info(),
                    to: ctx.accounts.rider_token.to_account_info(),
                    authority: ctx.accounts.bank_authority.to_account_info()
                }),
            pay
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hop<'info>{
    /// CHECK:
    pub rail_cell: AccountInfo<'info>,
    /// CHECK:
    pub rider: AccountInfo<'info>,
    /// CHECK:
    pub rail_hint: AccountInfo<'info>,
    /// CHECK:
    pub stage: AccountInfo<'info>,
    /// CHECK:
    pub cast_hint: AccountInfo<'info>,
    #[account(mut)]
    pub bank: Account<'info,TokenAccount>,
    #[account(mut)]
    pub rider_token: Account<'info,TokenAccount>,
    /// CHECK:
    pub bank_authority: AccountInfo<'info>,
    pub token_program: Program<'info,Token>,
}
