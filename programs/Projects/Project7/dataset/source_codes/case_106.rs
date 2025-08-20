// 6) スロット偶奇で Token と Token2022 のどちらかを呼ぶ
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer as Xfer, Token, TokenAccount};
use anchor_spl::token_2022::{self as token_2022, Transfer as Xfer22, Token2022};

declare_id!("AcctOnlyParityToken2022EEEEEEEEEEEEEEEEEEE");

#[program]
pub mod parity_token2022 {
    use super::*;
    pub fn init(ctx: Context<InitParity>, unit: u64) -> Result<()> {
        ctx.accounts.cfg.unit = unit.max(1);
        ctx.accounts.cfg.operator = ctx.accounts.operator.key();
        Ok(())
    }
    pub fn run(ctx: Context<RunParity>) -> Result<()> {
        let slot = Clock::get()?.slot;
        let odd = (slot % 2) != 0;

        if odd {
            token_2022::transfer(CpiContext::new(
                ctx.accounts.token2022_program.to_account_info(),
                Xfer22 {
                    from: ctx.accounts.a22.to_account_info(),
                    to: ctx.accounts.b22.to_account_info(),
                    authority: ctx.accounts.operator.to_account_info(),
                },
            ), ctx.accounts.cfg.unit)?;
        }
        if !odd {
            token::transfer(CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Xfer {
                    from: ctx.accounts.a_legacy.to_account_info(),
                    to: ctx.accounts.b_legacy.to_account_info(),
                    authority: ctx.accounts.operator.to_account_info(),
                },
            ), ctx.accounts.cfg.unit)?;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitParity<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8)]
    pub cfg: Account<'info, ParityCfg>,
    #[account(mut)] pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct RunParity<'info> {
    #[account(mut, has_one = operator)]
    pub cfg: Account<'info, ParityCfg>,
    pub operator: Signer<'info>,
    // legacy
    #[account(mut)] pub a_legacy: Account<'info, TokenAccount>,
    #[account(mut)] pub b_legacy: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    // 2022
    #[account(mut)] pub a22: Account<'info, token_2022::TokenAccount>,
    #[account(mut)] pub b22: Account<'info, token_2022::TokenAccount>,
    pub token2022_program: Program<'info, Token2022>,
}
#[account] pub struct ParityCfg { pub operator: Pubkey, pub unit: u64 }
