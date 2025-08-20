use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0990D83ID");

#[program]
pub mod airdrop_case_099 {
    use super::*;

    pub fn airdrop_action_099(ctx: Context<AirdropCtx099>, amount: u64) -> Result<()> {

        // Simple token transfer
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.airdrop_src_099.to_account_info(), to: ctx.accounts.airdrop_dst_099.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"airdrop", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AirdropCtx099<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub airdrop_src_099: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub airdrop_dst_099: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"airdrop"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



