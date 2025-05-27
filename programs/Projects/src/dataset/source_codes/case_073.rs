// Case 73: リミットオーダー発注
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u073eKfp");

#[program]
pub mod case_073 {
    use super::*;

// Case 73: リミットオーダー発注
pub fn execute_case_073(ctx: Context<ContextCase073>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.acct_src_073;
    let to = &mut ctx.accounts.acct_dst_073;
    // Missing signer: signer_073
    **from.to_account_info().try_borrow_mut_lamports()? -= amount;
    **to.to_account_info().try_borrow_mut_lamports()? += amount;
    msg!("Transferred {} lamports", amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase073<'info> {
    #[account(mut)]
    pub acct_src_073: Account<'info, TokenAccount>,
    /// CHECK: signer missing for リミットオーダー発注
    pub signer_073: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_073: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
