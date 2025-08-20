// Case 76: マーケット削除
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u076eKfp");

#[program]
pub mod case_076 {
    use super::*;

// Case 76: マーケット削除
pub fn execute_case_076(ctx: Context<ContextCase076>, stake_amount: u64) -> Result<()> {
    let staker = &mut ctx.accounts.acct_src_076;
    // Missing signer: signer_076
    staker.deposit(stake_amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase076<'info> {
    #[account(mut)]
    pub acct_src_076: Account<'info, TokenAccount>,
    /// CHECK: signer missing for マーケット削除
    pub signer_076: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_076: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
