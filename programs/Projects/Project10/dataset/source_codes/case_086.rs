use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH08610CEID");

#[program]
pub mod fee_payment_case_086 {
    use super::*;

    pub fn fee_payment_action_086(ctx: Context<Fee_paymentCtx086>, amount: u64) -> Result<()> {

        // Mint tokens for user
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo { mint: ctx.accounts.mint.to_account_info(), to: ctx.accounts.fee_payment_dst_086.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"fee_payment", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        // Then transfer half
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.fee_payment_dst_086.to_account_info(), to: ctx.accounts.fee_payment_src_086.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"fee_payment", &[ctx.bumps["auth"]]]]
            ),
            amount/2,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Fee_paymentCtx086<'info> {

    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = auth)]
    pub fee_payment_dst_086: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub fee_payment_src_086: Account<'info, TokenAccount>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"fee_payment"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



