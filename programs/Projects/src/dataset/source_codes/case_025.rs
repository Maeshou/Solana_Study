// Case 25: 定期購読停止
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u025eKfp");

#[program]
pub mod case_025 {
    use super::*;

// Case 25: 定期購読停止
pub fn execute_case_025(ctx: Context<ContextCase025>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.acct_src_025;
    let to = &mut ctx.accounts.acct_dst_025;
    // Missing signer: signer_025
    **from.to_account_info().try_borrow_mut_lamports()? -= amount;
    **to.to_account_info().try_borrow_mut_lamports()? += amount;
    msg!("Transferred {} lamports", amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase025<'info> {
    #[account(mut)]
    pub acct_src_025: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 定期購読停止
    pub signer_025: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_025: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
