// Case 35: バッチトランザクション実行
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u035eKfp");

#[program]
pub mod case_035 {
    use super::*;

// Case 35: バッチトランザクション実行
pub fn execute_case_035(ctx: Context<ContextCase035>) -> Result<()> {
    let claimer = &ctx.accounts.acct_src_035;
    // Missing signer: signer_035
    let reward = claimer.calculate_reward();
    claimer.receive(reward)?;
    msg!("Reward {} claimed", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase035<'info> {
    #[account(mut)]
    pub acct_src_035: Account<'info, TokenAccount>,
    /// CHECK: signer missing for バッチトランザクション実行
    pub signer_035: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_035: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
