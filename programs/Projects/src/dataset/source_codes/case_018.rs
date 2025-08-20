// Case 18: マルチシグ取引実行
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u018eKfp");

#[program]
pub mod case_018 {
    use super::*;

// Case 18: マルチシグ取引実行
pub fn execute_case_018(ctx: Context<ContextCase018>, amount: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.acct_src_018;
    let recipient = &mut ctx.accounts.acct_dst_018;
    // Missing signer: signer_018
    treasury.distribute(recipient.key(), amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase018<'info> {
    #[account(mut)]
    pub acct_src_018: Account<'info, TokenAccount>,
    /// CHECK: signer missing for マルチシグ取引実行
    pub signer_018: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_018: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
