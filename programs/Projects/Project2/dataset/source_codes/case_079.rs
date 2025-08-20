use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, SetAuthority, AuthorityType};

declare_id!("AuthFreeze99999999999999999999999999999999");

#[program]
pub mod authority_freeze {
    use super::*;

    pub fn freeze(ctx: Context<Freeze>, new_authority: Pubkey) -> Result<()> {
        let cpi_accounts = SetAuthority {
            account_or_mint: ctx.accounts.token.to_account_info(),
            current_authority: ctx.accounts.auth.to_account_info(),
        };
        let cpi = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        anchor_spl::token::set_authority(
            cpi,
            Some(new_authority),
            AuthorityType::FreezeAccount,
        )?;
        emit!(AccountFrozen { by: ctx.accounts.auth.key() });
        Ok(())
    }

    pub fn unfreeze(ctx: Context<Freeze>) -> Result<()> {
        let cpi_accounts = SetAuthority {
            account_or_mint: ctx.accounts.token.to_account_info(),
            current_authority: ctx.accounts.auth.to_account_info(),
        };
        let cpi = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        anchor_spl::token::set_authority(
            cpi,
            None,
            AuthorityType::FreezeAccount,
        )?;
        emit!(AccountUnfrozen { by: ctx.accounts.auth.key() });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Freeze<'info> {
    #[account(mut)]
    pub token: Account<'info, TokenAccount>,
    pub auth: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[event]
pub struct AccountFrozen {
    pub by: Pubkey,
}

#[event]
pub struct AccountUnfrozen {
    pub by: Pubkey,
}
