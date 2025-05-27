// Case 91: 資産担保解除
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u091eKfp");

#[program]
pub mod case_091 {
    use super::*;

// Case 91: 資産担保解除
pub fn execute_case_091(ctx: Context<ContextCase091>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.acct_src_091;
    let to = &mut ctx.accounts.acct_dst_091;
    // Missing signer: signer_091
    **from.to_account_info().try_borrow_mut_lamports()? -= amount;
    **to.to_account_info().try_borrow_mut_lamports()? += amount;
    msg!("Transferred {} lamports", amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase091<'info> {
    #[account(mut)]
    pub acct_src_091: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 資産担保解除
    pub signer_091: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_091: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
