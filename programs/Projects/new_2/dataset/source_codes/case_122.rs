use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, MintTo, Token};

declare_id!("MixChk3333333333333333333333333333333333");

#[program]
pub mod mixed_check3 {
    pub fn mint_and_log(ctx: Context<MintLog>) -> Result<()> {
        // mint の authority は検証あり
        let cpi = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to:   ctx.accounts.dest.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
        );
        anchor_spl::token::mint_to(cpi, 1)?;
        // log_acc は未検証
        let mut data = ctx.accounts.log_acc.data.borrow_mut();
        data.extend_from_slice(b"minted");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintLog<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub dest: Account<'info, anchor_spl::token::TokenAccount>,
    /// CHECK: authority is actually mint.authority
    pub mint_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,

    /// CHECK: raw account, owner check missing
    #[account(mut)]
    pub log_acc: AccountInfo<'info>,
}
