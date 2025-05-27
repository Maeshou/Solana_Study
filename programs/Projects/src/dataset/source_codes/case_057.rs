// Case 57: サブアカウント作成
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u057eKfp");

#[program]
pub mod case_057 {
    use super::*;

// Case 57: サブアカウント作成
pub fn execute_case_057(ctx: Context<ContextCase057>, burn_amount: u64) -> Result<()> {
    let from = &ctx.accounts.acct_src_057;
    // Missing signer on burn_authority: signer_057
    token::burn(ctx.accounts.into_burn_context(), burn_amount)?;
    msg!("Burned {} tokens", burn_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase057<'info> {
    #[account(mut)]
    pub acct_src_057: Account<'info, TokenAccount>,
    /// CHECK: signer missing for サブアカウント作成
    pub signer_057: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_057: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
