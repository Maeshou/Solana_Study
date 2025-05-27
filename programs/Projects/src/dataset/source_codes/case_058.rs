// Case 58: サブアカウント削除
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u058eKfp");

#[program]
pub mod case_058 {
    use super::*;

// Case 58: サブアカウント削除
pub fn execute_case_058(ctx: Context<ContextCase058>, stake_amount: u64) -> Result<()> {
    let staker = &mut ctx.accounts.acct_src_058;
    // Missing signer: signer_058
    staker.deposit(stake_amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase058<'info> {
    #[account(mut)]
    pub acct_src_058: Account<'info, TokenAccount>,
    /// CHECK: signer missing for サブアカウント削除
    pub signer_058: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_058: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
