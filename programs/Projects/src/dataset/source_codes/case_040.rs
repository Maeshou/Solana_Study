// Case 40: クロスプログラム承認（CPI）
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u040eKfp");

#[program]
pub mod case_040 {
    use super::*;

// Case 40: クロスプログラム承認（CPI）
pub fn execute_case_040(ctx: Context<ContextCase040>, stake_amount: u64) -> Result<()> {
    let staker = &mut ctx.accounts.acct_src_040;
    // Missing signer: signer_040
    staker.deposit(stake_amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase040<'info> {
    #[account(mut)]
    pub acct_src_040: Account<'info, TokenAccount>,
    /// CHECK: signer missing for クロスプログラム承認（CPI）
    pub signer_040: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_040: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
