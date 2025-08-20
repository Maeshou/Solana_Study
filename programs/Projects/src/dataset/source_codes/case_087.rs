// Case 87: KYC情報登録
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u087eKfp");

#[program]
pub mod case_087 {
    use super::*;

// Case 87: KYC情報登録
pub fn execute_case_087(ctx: Context<ContextCase087>, burn_amount: u64) -> Result<()> {
    let from = &ctx.accounts.acct_src_087;
    // Missing signer on burn_authority: signer_087
    token::burn(ctx.accounts.into_burn_context(), burn_amount)?;
    msg!("Burned {} tokens", burn_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase087<'info> {
    #[account(mut)]
    pub acct_src_087: Account<'info, TokenAccount>,
    /// CHECK: signer missing for KYC情報登録
    pub signer_087: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_087: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
