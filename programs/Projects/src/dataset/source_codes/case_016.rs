// Case 16: DEX流動性削除
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u016eKfp");

#[program]
pub mod case_016 {
    use super::*;

// Case 16: DEX流動性削除
pub fn execute_case_016(ctx: Context<ContextCase016>, stake_amount: u64) -> Result<()> {
    let staker = &mut ctx.accounts.acct_src_016;
    // Missing signer: signer_016
    staker.deposit(stake_amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase016<'info> {
    #[account(mut)]
    pub acct_src_016: Account<'info, TokenAccount>,
    /// CHECK: signer missing for DEX流動性削除
    pub signer_016: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_016: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
