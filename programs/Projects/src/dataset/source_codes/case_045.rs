// Case 45: プログラムハッシュ更新（アップグレード）
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u045eKfp");

#[program]
pub mod case_045 {
    use super::*;

// Case 45: プログラムハッシュ更新（アップグレード）
pub fn execute_case_045(ctx: Context<ContextCase045>, burn_amount: u64) -> Result<()> {
    let from = &ctx.accounts.acct_src_045;
    // Missing signer on burn_authority: signer_045
    token::burn(ctx.accounts.into_burn_context(), burn_amount)?;
    msg!("Burned {} tokens", burn_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase045<'info> {
    #[account(mut)]
    pub acct_src_045: Account<'info, TokenAccount>,
    /// CHECK: signer missing for プログラムハッシュ更新（アップグレード）
    pub signer_045: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_045: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
