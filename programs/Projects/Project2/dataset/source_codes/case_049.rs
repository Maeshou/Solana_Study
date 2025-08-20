use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, SetAuthority};

declare_id!("AuthFreeze99999999999999999999999999999999");

#[program]
pub mod authority_freeze {
    use super::*;

    pub fn freeze(ctx: Context<Freeze>) -> Result<()> {
        let cpi_accounts = SetAuthority {
            account_or_mint: ctx.accounts.token.to_account_info(),
            current_authority: ctx.accounts.auth.to_account_info(),
        };
        let cpi = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        anchor_spl::token::set_authority(
            cpi,
            Some(ctx.accounts.new_auth.key()),
            anchor_spl::token::AuthorityType::FreezeAccount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Freeze<'info> {
    #[account(mut)]
    pub token: Account<'info, TokenAccount>,
    pub auth: Signer<'info>,
    /// CHECK: 新権限はプログラムで決定
    pub new_auth: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
