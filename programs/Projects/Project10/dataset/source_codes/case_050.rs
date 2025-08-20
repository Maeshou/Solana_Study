use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH05011C1ID");

#[program]
pub mod staking_case_050 {
    use super::*;

    pub fn staking_action_050(ctx: Context<StakingCtx050>, amount: u64) -> Result<()> {

        // Simple token transfer
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.staking_src_050.to_account_info(), to: ctx.accounts.staking_dst_050.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"staking", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StakingCtx050<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub staking_src_050: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub staking_dst_050: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"staking"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



