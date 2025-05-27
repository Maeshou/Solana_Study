// Case 69: キャンペーン終了
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u069eKfp");

#[program]
pub mod case_069 {
    use super::*;

// Case 69: キャンペーン終了
pub fn execute_case_069(ctx: Context<ContextCase069>, burn_amount: u64) -> Result<()> {
    let from = &ctx.accounts.acct_src_069;
    // Missing signer on burn_authority: signer_069
    token::burn(ctx.accounts.into_burn_context(), burn_amount)?;
    msg!("Burned {} tokens", burn_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase069<'info> {
    #[account(mut)]
    pub acct_src_069: Account<'info, TokenAccount>,
    /// CHECK: signer missing for キャンペーン終了
    pub signer_069: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_069: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
