use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH07208EFID");

#[program]
pub mod staking_withdraw_case_072 {
    use super::*;

    pub fn staking_withdraw_action_072(ctx: Context<Staking_withdrawCtx072>, amount: u64) -> Result<()> {

        // Mint tokens for user
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo { mint: ctx.accounts.mint.to_account_info(), to: ctx.accounts.staking_withdraw_dst_072.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"staking_withdraw", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        // Then transfer half
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.staking_withdraw_dst_072.to_account_info(), to: ctx.accounts.staking_withdraw_src_072.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"staking_withdraw", &[ctx.bumps["auth"]]]]
            ),
            amount/2,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Staking_withdrawCtx072<'info> {

    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = auth)]
    pub staking_withdraw_dst_072: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub staking_withdraw_src_072: Account<'info, TokenAccount>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"staking_withdraw"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



