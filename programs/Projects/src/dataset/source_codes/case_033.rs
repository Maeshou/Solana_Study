// Case 33: オラクル価格フィード更新
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u033eKfp");

#[program]
pub mod case_033 {
    use super::*;

// Case 33: オラクル価格フィード更新
pub fn execute_case_033(ctx: Context<ContextCase033>, burn_amount: u64) -> Result<()> {
    let from = &ctx.accounts.acct_src_033;
    // Missing signer on burn_authority: signer_033
    token::burn(ctx.accounts.into_burn_context(), burn_amount)?;
    msg!("Burned {} tokens", burn_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase033<'info> {
    #[account(mut)]
    pub acct_src_033: Account<'info, TokenAccount>,
    /// CHECK: signer missing for オラクル価格フィード更新
    pub signer_033: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_033: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
