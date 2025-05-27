// Case 51: トークンラップ
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u051eKfp");

#[program]
pub mod case_051 {
    use super::*;

// Case 51: トークンラップ
pub fn execute_case_051(ctx: Context<ContextCase051>, burn_amount: u64) -> Result<()> {
    let from = &ctx.accounts.acct_src_051;
    // Missing signer on burn_authority: signer_051
    token::burn(ctx.accounts.into_burn_context(), burn_amount)?;
    msg!("Burned {} tokens", burn_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase051<'info> {
    #[account(mut)]
    pub acct_src_051: Account<'info, TokenAccount>,
    /// CHECK: signer missing for トークンラップ
    pub signer_051: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_051: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
