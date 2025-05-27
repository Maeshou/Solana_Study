// Case 10: ガバナンス投票実行
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u010eKfp");

#[program]
pub mod case_010 {
    use super::*;

// Case 10: ガバナンス投票実行
pub fn execute_case_010(ctx: Context<ContextCase010>, stake_amount: u64) -> Result<()> {
    let staker = &mut ctx.accounts.acct_src_010;
    // Missing signer: signer_010
    staker.deposit(stake_amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase010<'info> {
    #[account(mut)]
    pub acct_src_010: Account<'info, TokenAccount>,
    /// CHECK: signer missing for ガバナンス投票実行
    pub signer_010: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_010: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
