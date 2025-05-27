// Case 89: KYC情報削除
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u089eKfp");

#[program]
pub mod case_089 {
    use super::*;

// Case 89: KYC情報削除
pub fn execute_case_089(ctx: Context<ContextCase089>) -> Result<()> {
    let claimer = &ctx.accounts.acct_src_089;
    // Missing signer: signer_089
    let reward = claimer.calculate_reward();
    claimer.receive(reward)?;
    msg!("Reward {} claimed", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase089<'info> {
    #[account(mut)]
    pub acct_src_089: Account<'info, TokenAccount>,
    /// CHECK: signer missing for KYC情報削除
    pub signer_089: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_089: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
