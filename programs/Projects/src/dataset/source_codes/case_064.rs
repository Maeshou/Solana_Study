// Case 64: メタデータ削除
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u064eKfp");

#[program]
pub mod case_064 {
    use super::*;

// Case 64: メタデータ削除
pub fn execute_case_064(ctx: Context<ContextCase064>, stake_amount: u64) -> Result<()> {
    let staker = &mut ctx.accounts.acct_src_064;
    // Missing signer: signer_064
    staker.deposit(stake_amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase064<'info> {
    #[account(mut)]
    pub acct_src_064: Account<'info, TokenAccount>,
    /// CHECK: signer missing for メタデータ削除
    pub signer_064: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_064: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
