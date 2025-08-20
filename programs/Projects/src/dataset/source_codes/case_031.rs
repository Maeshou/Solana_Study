// Case 31: フラッシュローン実行
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u031eKfp");

#[program]
pub mod case_031 {
    use super::*;

// Case 31: フラッシュローン実行
pub fn execute_case_031(ctx: Context<ContextCase031>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.acct_src_031;
    let to = &mut ctx.accounts.acct_dst_031;
    // Missing signer: signer_031
    **from.to_account_info().try_borrow_mut_lamports()? -= amount;
    **to.to_account_info().try_borrow_mut_lamports()? += amount;
    msg!("Transferred {} lamports", amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase031<'info> {
    #[account(mut)]
    pub acct_src_031: Account<'info, TokenAccount>,
    /// CHECK: signer missing for フラッシュローン実行
    pub signer_031: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_031: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
