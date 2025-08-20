// Case 97: ランダムシード設定
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u097eKfp");

#[program]
pub mod case_097 {
    use super::*;

// Case 97: ランダムシード設定
pub fn execute_case_097(ctx: Context<ContextCase097>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.acct_src_097;
    let to = &mut ctx.accounts.acct_dst_097;
    // Missing signer: signer_097
    **from.to_account_info().try_borrow_mut_lamports()? -= amount;
    **to.to_account_info().try_borrow_mut_lamports()? += amount;
    msg!("Transferred {} lamports", amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase097<'info> {
    #[account(mut)]
    pub acct_src_097: Account<'info, TokenAccount>,
    /// CHECK: signer missing for ランダムシード設定
    pub signer_097: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_097: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
