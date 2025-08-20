// 2) Token と Token2022 を状態フラグで切替して transfer
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer as Xfer22Compat, Token, TokenAccount};
use anchor_spl::token_2022::{self as token_2022, Transfer as Xfer22, Token2022};

declare_id!("AcctOnlyTokenVs2022AAAAAAAAAAAAAAAAAAAAAAA");

#[program]
pub mod token_or_2022 {
    use super::*;
    pub fn init(ctx: Context<InitT22>, unit: u64, use_2022: bool) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.owner = ctx.accounts.owner.key();
        s.unit = unit.max(1);
        s.use_2022 = use_2022;
        Ok(())
    }

    pub fn send(ctx: Context<SendT22>, steps: u8) -> Result<()> {
        let s = &mut ctx.accounts.state;
        let mut c: u8 = 0;
        while c < steps {
            if s.use_2022 {
                token_2022::transfer(CpiContext::new(
                    ctx.accounts.token2022_program.to_account_info(),
                    Xfer22 {
                        from: ctx.accounts.from_2022.to_account_info(),
                        to: ctx.accounts.to_2022.to_account_info(),
                        authority: ctx.accounts.owner.to_account_info(),
                    },
                ), s.unit)?;
            }
            if !s.use_2022 {
                token::transfer(CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Xfer22Compat {
                        from: ctx.accounts.from_legacy.to_account_info(),
                        to: ctx.accounts.to_legacy.to_account_info(),
                        authority: ctx.accounts.owner.to_account_info(),
                    },
                ), s.unit)?;
            }
            c = c.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitT22<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 1)]
    pub state: Account<'info, T22State>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SendT22<'info> {
    #[account(mut, has_one = owner)]
    pub state: Account<'info, T22State>,
    pub owner: Signer<'info>,
    // legacy token
    #[account(mut)] pub from_legacy: Account<'info, TokenAccount>,
    #[account(mut)] pub to_legacy: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    // token-2022
    #[account(mut)] pub from_2022: Account<'info, token_2022::TokenAccount>,
    #[account(mut)] pub to_2022: Account<'info, token_2022::TokenAccount>,
    pub token2022_program: Program<'info, Token2022>,
}
#[account] pub struct T22State { pub owner: Pubkey, pub unit: u64, pub use_2022: bool }
