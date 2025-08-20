// Case 52: トークンアンラップ
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u052eKfp");

#[program]
pub mod case_052 {
    use super::*;

// Case 52: トークンアンラップ
pub fn execute_case_052(ctx: Context<ContextCase052>, stake_amount: u64) -> Result<()> {
    let staker = &mut ctx.accounts.acct_src_052;
    // Missing signer: signer_052
    staker.deposit(stake_amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase052<'info> {
    #[account(mut)]
    pub acct_src_052: Account<'info, TokenAccount>,
    /// CHECK: signer missing for トークンアンラップ
    pub signer_052: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_052: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
