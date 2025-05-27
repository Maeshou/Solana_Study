// Case 82: 期間ロック解除
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u082eKfp");

#[program]
pub mod case_082 {
    use super::*;

// Case 82: 期間ロック解除
pub fn execute_case_082(ctx: Context<ContextCase082>, stake_amount: u64) -> Result<()> {
    let staker = &mut ctx.accounts.acct_src_082;
    // Missing signer: signer_082
    staker.deposit(stake_amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase082<'info> {
    #[account(mut)]
    pub acct_src_082: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 期間ロック解除
    pub signer_082: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_082: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
