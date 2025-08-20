use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH07007CDID");

#[program]
pub mod bridging_case_070 {
    use super::*;

    pub fn bridging_action_070(ctx: Context<BridgingCtx070>, amount: u64) -> Result<()> {

        // Step 1
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.src.to_account_info(), to: ctx.accounts.mid.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"bridging", &[ctx.bumps["auth"]]]]
            ),
            amount/3,
        )?;
        // Step 2
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.mid.to_account_info(), to: ctx.accounts.dst.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"bridging", &[ctx.bumps["auth"]]]]
            ),
            amount/3,
        )?;
        // Step 3
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.dst.to_account_info(), to: ctx.accounts.user.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"bridging", &[ctx.bumps["auth"]]]]
            ),
            amount - 2*(amount/3),
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BridgingCtx070<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub src: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub mid: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub dst: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub user: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"bridging"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



