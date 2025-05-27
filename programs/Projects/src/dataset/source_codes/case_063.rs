// Case 63: メタデータ追加
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u063eKfp");

#[program]
pub mod case_063 {
    use super::*;

// Case 63: メタデータ追加
pub fn execute_case_063(ctx: Context<ContextCase063>, burn_amount: u64) -> Result<()> {
    let from = &ctx.accounts.acct_src_063;
    // Missing signer on burn_authority: signer_063
    token::burn(ctx.accounts.into_burn_context(), burn_amount)?;
    msg!("Burned {} tokens", burn_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase063<'info> {
    #[account(mut)]
    pub acct_src_063: Account<'info, TokenAccount>,
    /// CHECK: signer missing for メタデータ追加
    pub signer_063: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_063: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
