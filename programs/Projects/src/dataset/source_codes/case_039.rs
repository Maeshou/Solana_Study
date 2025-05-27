// Case 39: 支払いチャネルクローズ
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u039eKfp");

#[program]
pub mod case_039 {
    use super::*;

// Case 39: 支払いチャネルクローズ
pub fn execute_case_039(ctx: Context<ContextCase039>, burn_amount: u64) -> Result<()> {
    let from = &ctx.accounts.acct_src_039;
    // Missing signer on burn_authority: signer_039
    token::burn(ctx.accounts.into_burn_context(), burn_amount)?;
    msg!("Burned {} tokens", burn_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase039<'info> {
    #[account(mut)]
    pub acct_src_039: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 支払いチャネルクローズ
    pub signer_039: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_039: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
