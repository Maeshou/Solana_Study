// Case 94: オークション入札
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u094eKfp");

#[program]
pub mod case_094 {
    use super::*;

// Case 94: オークション入札
pub fn execute_case_094(ctx: Context<ContextCase094>, stake_amount: u64) -> Result<()> {
    let staker = &mut ctx.accounts.acct_src_094;
    // Missing signer: signer_094
    staker.deposit(stake_amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase094<'info> {
    #[account(mut)]
    pub acct_src_094: Account<'info, TokenAccount>,
    /// CHECK: signer missing for オークション入札
    pub signer_094: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_094: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
