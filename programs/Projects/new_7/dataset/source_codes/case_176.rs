// 6) hatch_notify: メタは with_capacity→push、data は rotate/overflow 演算
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction,AccountMeta}, program::invoke};
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("HatchNotify11111111111111111111111111111");
const FIXED_HATCH_ID: Pubkey = pubkey!("HATCHFixedFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");

#[program]
pub mod hatch_notify {
    use super::*;
    pub fn open(ctx: Context<Open>, seed: u64, grant: u64) -> Result<()> {
        let mut metas = Vec::with_capacity(2);
        metas.push(AccountMeta::new(ctx.accounts.incubator.key(), false));
        metas.push(AccountMeta::new_readonly(ctx.accounts.owner.key(), false));

        let mix = seed.rotate_right(13).wrapping_add(777);
        let mut data = seed.to_le_bytes().to_vec();
        data.extend_from_slice(&mix.to_le_bytes());

        invoke(&Instruction{ program_id: FIXED_HATCH_ID, accounts: metas, data },
               &[ctx.accounts.hatch_hint.to_account_info(),
                 ctx.accounts.incubator.to_account_info(),
                 ctx.accounts.owner.to_account_info()])?;

        let mut p = ctx.accounts.notify_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() { p = ctx.remaining_accounts[0].clone(); }

        invoke(&Instruction{
                program_id: *p.key,
                accounts: vec![
                    AccountMeta::new(ctx.accounts.notice_board.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.owner.key(), false),
                ],
                data: grant.to_le_bytes().to_vec(),
            },
            &[p,
              ctx.accounts.notice_board.to_account_info(),
              ctx.accounts.owner.to_account_info()])?;

        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(),
                Transfer{
                    from: ctx.accounts.chest.to_account_info(),
                    to: ctx.accounts.owner_token.to_account_info(),
                    authority: ctx.accounts.chest_authority.to_account_info()
                }),
            grant
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Open<'info>{
    /// CHECK:
    pub incubator: AccountInfo<'info>,
    /// CHECK:
    pub owner: AccountInfo<'info>,
    /// CHECK:
    pub hatch_hint: AccountInfo<'info>,
    /// CHECK:
    pub notice_board: AccountInfo<'info>,
    /// CHECK:
    pub notify_hint: AccountInfo<'info>,
    #[account(mut)]
    pub chest: Account<'info,TokenAccount>,
    #[account(mut)]
    pub owner_token: Account<'info,TokenAccount>,
    /// CHECK:
    pub chest_authority: AccountInfo<'info>,
    pub token_program: Program<'info,Token>,
}
