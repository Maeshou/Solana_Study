use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0221112ID");

#[program]
pub mod bridging_case_022 {
    use super::*;

    pub fn bridging_action_022(ctx: Context<BridgingCtx022>, amount: u64) -> Result<()> {

        // Simple token transfer
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.bridging_src_022.to_account_info(), to: ctx.accounts.bridging_dst_022.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"bridging", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BridgingCtx022<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub bridging_src_022: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub bridging_dst_022: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"bridging"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



