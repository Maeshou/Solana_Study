// Case 70: クーポン発行
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u070eKfp");

#[program]
pub mod case_070 {
    use super::*;

// Case 70: クーポン発行
pub fn execute_case_070(ctx: Context<ContextCase070>, stake_amount: u64) -> Result<()> {
    let staker = &mut ctx.accounts.acct_src_070;
    // Missing signer: signer_070
    staker.deposit(stake_amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase070<'info> {
    #[account(mut)]
    pub acct_src_070: Account<'info, TokenAccount>,
    /// CHECK: signer missing for クーポン発行
    pub signer_070: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_070: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
