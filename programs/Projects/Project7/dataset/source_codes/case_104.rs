// 4) 同一関数内で Token approve と System transfer を順序付きで選択
use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer as SysTransfer};
use anchor_spl::token::{self, Approve as SplApprove, Token, TokenAccount};

declare_id!("AcctOnlyApproveOrSysCCCCCCCCCCCCCCCCCCCCCC");

#[program]
pub mod approve_or_sys {
    use super::*;
    pub fn run(ctx: Context<RunAoS>, do_approve: bool, lamports: u64, tokens: u64) -> Result<()> {
        if do_approve {
            token::approve(CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                SplApprove {
                    to: ctx.accounts.holder.to_account_info(),
                    delegate: ctx.accounts.spender.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ), tokens)?;
        }
        if !do_approve {
            system_program::transfer(CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                SysTransfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.recipient.to_account_info(),
                },
            ), lamports)?;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RunAoS<'info> {
    pub owner: Signer<'info>,
    // approve 用
    #[account(mut)] pub holder: Account<'info, TokenAccount>,
    #[account(mut)] pub spender: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    // system transfer 用
    #[account(mut)] pub payer: Signer<'info>,
    #[account(mut)] pub recipient: Account<'info, SystemAccount>,
    pub system_program: Program<'info, System>,
}
