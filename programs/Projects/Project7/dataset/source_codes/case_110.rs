// 10) 窓位置によって System/Token/Token2022 を段階的に切替
use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer as SysTransfer};
use anchor_spl::token::{self, Transfer as Xfer, Token, TokenAccount};
use anchor_spl::token_2022::{self as token_2022, Transfer as Xfer22, Token2022};

declare_id!("AcctOnlyWindowedIIIIIIIIIIIIIIIIIIIIIIIIII");

#[program]
pub mod windowed_router {
    use super::*;
    pub fn init(ctx: Context<InitW>, unit_sol: u64, unit_spl: u64, window: u64) -> Result<()> {
        let s = &mut ctx.accounts.switcher;
        s.operator = ctx.accounts.operator.key();
        s.unit_sol = unit_sol;
        s.unit_spl = unit_spl.max(1);
        s.window = window.max(3);
        s.pos = 0;
        Ok(())
    }
    pub fn tick(ctx: Context<TickW>) -> Result<()> {
        let s = &mut ctx.accounts.switcher;
        let idx = s.pos % s.window;

        if idx == 0 {
            system_program::transfer(CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                SysTransfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.treasury.to_account_info(),
                },
            ), s.unit_sol)?;
        }
        if idx == 1 {
            token::transfer(CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Xfer {
                    from: ctx.accounts.legacy_from.to_account_info(),
                    to: ctx.accounts.legacy_to.to_account_info(),
                    authority: ctx.accounts.operator.to_account_info(),
                },
            ), s.unit_spl)?;
        }
        if idx >= 2 {
            token_2022::transfer(CpiContext::new(
                ctx.accounts.token2022_program.to_account_info(),
                Xfer22 {
                    from: ctx.accounts.t22_from.to_account_info(),
                    to: ctx.accounts.t22_to.to_account_info(),
                    authority: ctx.accounts.operator.to_account_info(),
                },
            ), s.unit_spl)?;
        }

        s.pos = s.pos.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitW<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub switcher: Account<'info, WindowedState>,
    #[account(mut)] pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct TickW<'info> {
    #[account(mut, has_one = operator)]
    pub switcher: Account<'info, WindowedState>,
    pub operator: Signer<'info>,
    // system
    #[account(mut)] pub payer: Signer<'info>,
    #[account(mut)] pub treasury: Account<'info, SystemAccount>,
    pub system_program: Program<'info, System>,
    // token legacy
    #[account(mut)] pub legacy_from: Account<'info, TokenAccount>,
    #[account(mut)] pub legacy_to: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    // token 2022
    #[account(mut)] pub t22_from: Account<'info, token_2022::TokenAccount>,
    #[account(mut)] pub t22_to: Account<'info, token_2022::TokenAccount>,
    pub token2022_program: Program<'info, Token2022>,
}
#[account] pub struct WindowedState { pub operator: Pubkey, pub unit_sol: u64, pub unit_spl: u64, pub window: u64, pub pos: u64 }
