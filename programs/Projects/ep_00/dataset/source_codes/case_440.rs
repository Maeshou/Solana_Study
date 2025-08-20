use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Transfer, CpiContext, Token};

declare_id!("Secu44044444444444444444444444444444444");

#[program]
mod case_440 {
    use super::*;

    pub fn process_440(ctx: Context<Ctx440>, amount: u64) -> Result<()> {
        let from_t = &ctx.accounts.from_token;
        let to_t = &ctx.accounts.to_token;
        require!(from_t.key() != to_t.key(), ErrorCode::DuplicateToken);
        let owner_key = ctx.accounts.owner.key();
        require!(owner_key == from_t.owner, ErrorCode::BadOwner);

        let before = from_t.amount;
        require!(before >= amount, ErrorCode::NotEnoughTokens);
        let cpi_accounts = Transfer {
            from: from_t.to_account_info(),
            to: to_t.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_prog.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        let after = from_t.reload()?.amount;
        let diff = before.checked_sub(after).unwrap_or(0);
        let _ = diff * 7; // dummy computation to vary code
        msg!("{} tokens moved", diff);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx440<'info> {
    #[account(mut)]
    pub from_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_token: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
    pub token_prog: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Token accounts must differ")]
    DuplicateToken,
    #[msg("Invalid token owner")]
    BadOwner,
    #[msg("Not enough tokens")]
    NotEnoughTokens,
}
