use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0731CBEID");

#[program]
pub mod subscription_case_073 {
    use super::*;

    pub fn subscription_action_073(ctx: Context<SubscriptionCtx073>, amount: u64) -> Result<()> {

        // Burn some tokens
        token::burn(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Burn { mint: ctx.accounts.mint.to_account_info(), to: ctx.accounts.subscription_src_073.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"subscription", &[ctx.bumps["auth"]]]]
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
pub struct SubscriptionCtx073<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub subscription_src_073: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub state: Account<'info, DataState>,

    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"subscription"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}


#[account]
pub struct DataState {
    pub count: u64,
}


