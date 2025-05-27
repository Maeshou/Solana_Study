// Case 100: ライフサイクルイベントトリガー
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u100eKfp");

#[program]
pub mod case_100 {
    use super::*;

// Case 100: ライフサイクルイベントトリガー
pub fn execute_case_100(ctx: Context<ContextCase100>, stake_amount: u64) -> Result<()> {
    let staker = &mut ctx.accounts.acct_src_100;
    // Missing signer: signer_100
    staker.deposit(stake_amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase100<'info> {
    #[account(mut)]
    pub acct_src_100: Account<'info, TokenAccount>,
    /// CHECK: signer missing for ライフサイクルイベントトリガー
    pub signer_100: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_100: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
