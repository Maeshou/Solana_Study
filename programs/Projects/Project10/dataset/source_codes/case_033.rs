use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0330420ID");

#[program]
pub mod lottery_case_033 {
    use super::*;

    pub fn lottery_action_033(ctx: Context<LotteryCtx033>, amount: u64) -> Result<()> {

        // Transfer and emit event
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.pool.to_account_info(), to: ctx.accounts.user.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"lottery", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        emit!(TransferEvent { user: ctx.accounts.user.key(), amount });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LotteryCtx033<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub pool: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub user: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"lottery"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



#[event]
pub struct TransferEvent {
    pub user: Pubkey,
    pub amount: u64,
}

