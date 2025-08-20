use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};

use anchor_lang::solana_program::clock::Clock;
declare_id!("PDSH06718A0ID");

#[program]
pub mod tipping_case_067 {
    use super::*;

    pub fn tipping_action_067(ctx: Context<TippingCtx067>, amount: u64) -> Result<()> {

        // Charge subscription
        let fee = amount/10;
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.payer.to_account_info(), to: ctx.accounts.vault.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"tipping", &[ctx.bumps["auth"]]]]
            ),
            fee,
        )?;
        // Update expiry
        let now = Clock::get()?.unix_timestamp;
        ctx.accounts.sub_info.expires = now + 30*24*3600;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TippingCtx067<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub payer: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub sub_info: Account<'info, SubscriptionInfo>,
    pub mint: Account<'info, Mint>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"tipping"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}


#[account]
pub struct SubscriptionInfo {
    pub expires: i64,
}


