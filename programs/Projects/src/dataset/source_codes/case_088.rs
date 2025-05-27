// Case 88: KYC情報更新
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u088eKfp");

#[program]
pub mod case_088 {
    use super::*;

// Case 88: KYC情報更新
pub fn execute_case_088(ctx: Context<ContextCase088>, stake_amount: u64) -> Result<()> {
    let staker = &mut ctx.accounts.acct_src_088;
    // Missing signer: signer_088
    staker.deposit(stake_amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase088<'info> {
    #[account(mut)]
    pub acct_src_088: Account<'info, TokenAccount>,
    /// CHECK: signer missing for KYC情報更新
    pub signer_088: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_088: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
