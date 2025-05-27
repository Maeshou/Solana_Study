// Case 75: マーケット作成
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u075eKfp");

#[program]
pub mod case_075 {
    use super::*;

// Case 75: マーケット作成
pub fn execute_case_075(ctx: Context<ContextCase075>, burn_amount: u64) -> Result<()> {
    let from = &ctx.accounts.acct_src_075;
    // Missing signer on burn_authority: signer_075
    token::burn(ctx.accounts.into_burn_context(), burn_amount)?;
    msg!("Burned {} tokens", burn_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase075<'info> {
    #[account(mut)]
    pub acct_src_075: Account<'info, TokenAccount>,
    /// CHECK: signer missing for マーケット作成
    pub signer_075: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_075: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
