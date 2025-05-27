// Case 34: 手数料率変更
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u034eKfp");

#[program]
pub mod case_034 {
    use super::*;

// Case 34: 手数料率変更
pub fn execute_case_034(ctx: Context<ContextCase034>, stake_amount: u64) -> Result<()> {
    let staker = &mut ctx.accounts.acct_src_034;
    // Missing signer: signer_034
    staker.deposit(stake_amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase034<'info> {
    #[account(mut)]
    pub acct_src_034: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 手数料率変更
    pub signer_034: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_034: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
