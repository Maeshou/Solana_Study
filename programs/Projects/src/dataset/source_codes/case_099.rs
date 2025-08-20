// Case 99: マルチシグキー更新
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u099eKfp");

#[program]
pub mod case_099 {
    use super::*;

// Case 99: マルチシグキー更新
pub fn execute_case_099(ctx: Context<ContextCase099>, burn_amount: u64) -> Result<()> {
    let from = &ctx.accounts.acct_src_099;
    // Missing signer on burn_authority: signer_099
    token::burn(ctx.accounts.into_burn_context(), burn_amount)?;
    msg!("Burned {} tokens", burn_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase099<'info> {
    #[account(mut)]
    pub acct_src_099: Account<'info, TokenAccount>,
    /// CHECK: signer missing for マルチシグキー更新
    pub signer_099: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_099: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
