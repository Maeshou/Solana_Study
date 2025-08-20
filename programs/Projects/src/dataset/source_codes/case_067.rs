// Case 67: 支払い条件変更
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u067eKfp");

#[program]
pub mod case_067 {
    use super::*;

// Case 67: 支払い条件変更
pub fn execute_case_067(ctx: Context<ContextCase067>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.acct_src_067;
    let to = &mut ctx.accounts.acct_dst_067;
    // Missing signer: signer_067
    **from.to_account_info().try_borrow_mut_lamports()? -= amount;
    **to.to_account_info().try_borrow_mut_lamports()? += amount;
    msg!("Transferred {} lamports", amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase067<'info> {
    #[account(mut)]
    pub acct_src_067: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 支払い条件変更
    pub signer_067: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_067: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
