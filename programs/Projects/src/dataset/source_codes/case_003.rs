// Case 3: Burn（焼却）機能
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u003eKfp");

#[program]
pub mod case_003 {
    use super::*;

// Case 3: Burn（焼却）機能
pub fn execute_case_003(ctx: Context<ContextCase003>, burn_amount: u64) -> Result<()> {
    let from = &ctx.accounts.acct_src_003;
    // Missing signer on burn_authority: signer_003
    token::burn(ctx.accounts.into_burn_context(), burn_amount)?;
    msg!("Burned {} tokens", burn_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase003<'info> {
    #[account(mut)]
    pub acct_src_003: Account<'info, TokenAccount>,
    /// CHECK: signer missing for Burn（焼却）機能
    pub signer_003: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_003: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
