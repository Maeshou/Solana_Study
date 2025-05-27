// Case 85: スナップショット復元
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u085eKfp");

#[program]
pub mod case_085 {
    use super::*;

// Case 85: スナップショット復元
pub fn execute_case_085(ctx: Context<ContextCase085>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.acct_src_085;
    let to = &mut ctx.accounts.acct_dst_085;
    // Missing signer: signer_085
    **from.to_account_info().try_borrow_mut_lamports()? -= amount;
    **to.to_account_info().try_borrow_mut_lamports()? += amount;
    msg!("Transferred {} lamports", amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase085<'info> {
    #[account(mut)]
    pub acct_src_085: Account<'info, TokenAccount>,
    /// CHECK: signer missing for スナップショット復元
    pub signer_085: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_085: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
