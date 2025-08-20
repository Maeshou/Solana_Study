// Case 24: 定期購読支払い
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u024eKfp");

#[program]
pub mod case_024 {
    use super::*;

// Case 24: 定期購読支払い
pub fn execute_case_024(ctx: Context<ContextCase024>, amount: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.acct_src_024;
    let recipient = &mut ctx.accounts.acct_dst_024;
    // Missing signer: signer_024
    treasury.distribute(recipient.key(), amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase024<'info> {
    #[account(mut)]
    pub acct_src_024: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 定期購読支払い
    pub signer_024: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_024: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
