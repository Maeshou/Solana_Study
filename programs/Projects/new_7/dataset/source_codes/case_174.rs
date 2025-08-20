// 4) mint_mark: accounts を固定配列→into()、可変長 data に length 先頭付与
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction,AccountMeta}, program::invoke};
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("M1ntMark1111111111111111111111111111111");
const FIXED_MARK_ID: Pubkey = pubkey!("MARKFixedDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD");

#[program]
pub mod mint_mark {
    use super::*;
    pub fn note(ctx: Context<Note>, value: u64, pay: u64) -> Result<()> {
        let mut d = value.to_le_bytes().to_vec();
        d.insert(0, 8); // バイト長を軽く付与

        invoke(&Instruction{
                program_id: FIXED_MARK_ID,
                accounts: [
                    AccountMeta::new(ctx.accounts.mark_slot.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.user.key(), false),
                ].into(),
                data: d,
            },
            &[ctx.accounts.mark_hint.to_account_info(),
              ctx.accounts.mark_slot.to_account_info(),
              ctx.accounts.user.to_account_info()])?;

        let mut prog = ctx.accounts.push_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() { prog = ctx.remaining_accounts[0].clone(); }

        invoke(&Instruction{
                program_id: *prog.key,
                accounts: vec![
                    AccountMeta::new(ctx.accounts.stream.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.user.key(), false),
                ],
                data: pay.to_le_bytes().to_vec(),
            },
            &[prog,
              ctx.accounts.stream.to_account_info(),
              ctx.accounts.user.to_account_info()])?;

        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(),
                Transfer{
                    from: ctx.accounts.reserve.to_account_info(),
                    to: ctx.accounts.user_token.to_account_info(),
                    authority: ctx.accounts.reserve_authority.to_account_info()
                }),
            pay
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Note<'info>{
    /// CHECK:
    pub mark_slot: AccountInfo<'info>,
    /// CHECK:
    pub user: AccountInfo<'info>,
    /// CHECK:
    pub mark_hint: AccountInfo<'info>,
    /// CHECK:
    pub stream: AccountInfo<'info>,
    /// CHECK:
    pub push_hint: AccountInfo<'info>,
    #[account(mut)]
    pub reserve: Account<'info,TokenAccount>,
    #[account(mut)]
    pub user_token: Account<'info,TokenAccount>,
    /// CHECK:
    pub reserve_authority: AccountInfo<'info>,
    pub token_program: Program<'info,Token>,
}
