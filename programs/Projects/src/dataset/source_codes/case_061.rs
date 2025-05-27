// Case 61: ログレベル変更
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u061eKfp");

#[program]
pub mod case_061 {
    use super::*;

// Case 61: ログレベル変更
pub fn execute_case_061(ctx: Context<ContextCase061>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.acct_src_061;
    let to = &mut ctx.accounts.acct_dst_061;
    // Missing signer: signer_061
    **from.to_account_info().try_borrow_mut_lamports()? -= amount;
    **to.to_account_info().try_borrow_mut_lamports()? += amount;
    msg!("Transferred {} lamports", amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase061<'info> {
    #[account(mut)]
    pub acct_src_061: Account<'info, TokenAccount>,
    /// CHECK: signer missing for ログレベル変更
    pub signer_061: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_061: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
