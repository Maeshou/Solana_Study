use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH06419CAID");

#[program]
pub mod dividend_case_064 {
    use super::*;

    pub fn dividend_action_064(ctx: Context<DividendCtx064>, amount: u64) -> Result<()> {

        // Simple token transfer
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.dividend_src_064.to_account_info(), to: ctx.accounts.dividend_dst_064.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"dividend", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DividendCtx064<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub dividend_src_064: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub dividend_dst_064: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"dividend"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



