// Case 21: 管理者ロール割当
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u021eKfp");

#[program]
pub mod case_021 {
    use super::*;

// Case 21: 管理者ロール割当
pub fn execute_case_021(ctx: Context<ContextCase021>, burn_amount: u64) -> Result<()> {
    let from = &ctx.accounts.acct_src_021;
    // Missing signer on burn_authority: signer_021
    token::burn(ctx.accounts.into_burn_context(), burn_amount)?;
    msg!("Burned {} tokens", burn_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase021<'info> {
    #[account(mut)]
    pub acct_src_021: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 管理者ロール割当
    pub signer_021: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_021: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
