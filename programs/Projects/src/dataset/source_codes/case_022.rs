// Case 22: 管理者ロール剥奪
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u022eKfp");

#[program]
pub mod case_022 {
    use super::*;

// Case 22: 管理者ロール剥奪
pub fn execute_case_022(ctx: Context<ContextCase022>, stake_amount: u64) -> Result<()> {
    let staker = &mut ctx.accounts.acct_src_022;
    // Missing signer: signer_022
    staker.deposit(stake_amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase022<'info> {
    #[account(mut)]
    pub acct_src_022: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 管理者ロール剥奪
    pub signer_022: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_022: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
