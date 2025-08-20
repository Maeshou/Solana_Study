// 9) 面倒見フラグに応じて Token の approve か Token2022 の revoke を選択
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve as SplApprove, Revoke as SplRevoke, Token, TokenAccount};
use anchor_spl::token_2022::{self as token_2022, Revoke as Revoke22, Token2022};

declare_id!("AcctOnlyApproveOrRevoke2022HHHHHHHHHHHHHH");

#[program]
pub mod approve_or_revoke_2022 {
    use super::*;
    pub fn init(ctx: Context<InitAr>, approve_mode: bool, amount: u64) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.keeper = ctx.accounts.keeper.key();
        s.approve_mode = approve_mode;
        s.amount = amount.max(1);
        Ok(())
    }
    pub fn act(ctx: Context<ActAr>) -> Result<()> {
        if ctx.accounts.state.approve_mode {
            token::approve(CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                SplApprove {
                    to: ctx.accounts.holder.to_account_info(),
                    delegate: ctx.accounts.spender.to_account_info(),
                    authority: ctx.accounts.keeper.to_account_info(),
                },
            ), ctx.accounts.state.amount)?;
        }
        if !ctx.accounts.state.approve_mode {
            token_2022::revoke(CpiContext::new(
                ctx.accounts.token2022_program.to_account_info(),
                Revoke22 {
                    source: ctx.accounts.holder22.to_account_info(),
                    authority: ctx.accounts.keeper.to_account_info(),
                },
            ))?;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAr<'info> {
    #[account(init, payer = keeper, space = 8 + 32 + 1 + 8)]
    pub state: Account<'info, ArState>,
    #[account(mut)] pub keeper: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActAr<'info> {
    #[account(mut, has_one = keeper)]
    pub state: Account<'info, ArState>,
    pub keeper: Signer<'info>,
    // approve 用
    #[account(mut)] pub holder: Account<'info, TokenAccount>,
    #[account(mut)] pub spender: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    // revoke 用 (2022)
    #[account(mut)] pub holder22: Account<'info, token_2022::TokenAccount>,
    pub token2022_program: Program<'info, Token2022>,
}
#[account] pub struct ArState { pub keeper: Pubkey, pub approve_mode: bool, pub amount: u64 }
