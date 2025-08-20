// Case 28: DAOトレジャリー出金
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u028eKfp");

#[program]
pub mod case_028 {
    use super::*;

// Case 28: DAOトレジャリー出金
pub fn execute_case_028(ctx: Context<ContextCase028>, stake_amount: u64) -> Result<()> {
    let staker = &mut ctx.accounts.acct_src_028;
    // Missing signer: signer_028
    staker.deposit(stake_amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase028<'info> {
    #[account(mut)]
    pub acct_src_028: Account<'info, TokenAccount>,
    /// CHECK: signer missing for DAOトレジャリー出金
    pub signer_028: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_028: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
