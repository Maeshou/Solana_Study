// Case 77: インデックスファンド作成
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u077eKfp");

#[program]
pub mod case_077 {
    use super::*;

// Case 77: インデックスファンド作成
pub fn execute_case_077(ctx: Context<ContextCase077>) -> Result<()> {
    let claimer = &ctx.accounts.acct_src_077;
    // Missing signer: signer_077
    let reward = claimer.calculate_reward();
    claimer.receive(reward)?;
    msg!("Reward {} claimed", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase077<'info> {
    #[account(mut)]
    pub acct_src_077: Account<'info, TokenAccount>,
    /// CHECK: signer missing for インデックスファンド作成
    pub signer_077: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_077: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
