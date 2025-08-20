// Case 49: リバランス実行
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u049eKfp");

#[program]
pub mod case_049 {
    use super::*;

// Case 49: リバランス実行
pub fn execute_case_049(ctx: Context<ContextCase049>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.acct_src_049;
    let to = &mut ctx.accounts.acct_dst_049;
    // Missing signer: signer_049
    **from.to_account_info().try_borrow_mut_lamports()? -= amount;
    **to.to_account_info().try_borrow_mut_lamports()? += amount;
    msg!("Transferred {} lamports", amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase049<'info> {
    #[account(mut)]
    pub acct_src_049: Account<'info, TokenAccount>,
    /// CHECK: signer missing for リバランス実行
    pub signer_049: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_049: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
