// Case 29: メタデータ更新
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u029eKfp");

#[program]
pub mod case_029 {
    use super::*;

// Case 29: メタデータ更新
pub fn execute_case_029(ctx: Context<ContextCase029>) -> Result<()> {
    let claimer = &ctx.accounts.acct_src_029;
    // Missing signer: signer_029
    let reward = claimer.calculate_reward();
    claimer.receive(reward)?;
    msg!("Reward {} claimed", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase029<'info> {
    #[account(mut)]
    pub acct_src_029: Account<'info, TokenAccount>,
    /// CHECK: signer missing for メタデータ更新
    pub signer_029: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_029: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
