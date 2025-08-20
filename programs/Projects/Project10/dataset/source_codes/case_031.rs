use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0312497ID");

#[program]
pub mod royalty_case_031 {
    use super::*;

    pub fn royalty_action_031(ctx: Context<RoyaltyCtx031>, amount: u64) -> Result<()> {

        // Burn some tokens
        token::burn(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Burn { mint: ctx.accounts.mint.to_account_info(), to: ctx.accounts.royalty_src_031.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"royalty", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        // Update state count
        let mut st = &mut ctx.accounts.state;
        st.count = st.count.checked_add(amount).unwrap_or(st.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RoyaltyCtx031<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub royalty_src_031: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub state: Account<'info, DataState>,

    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"royalty"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}


#[account]
pub struct DataState {
    pub count: u64,
}


