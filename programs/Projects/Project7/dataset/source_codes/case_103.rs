// 3) Associated Token の create と Token transfer を分岐実行
use anchor_lang::prelude::*;
use anchor_spl::associated_token::{self, AssociatedToken, Create};
use anchor_spl::token::{self, Transfer as SplTransfer, Token, TokenAccount};

declare_id!("AcctOnlyATokenCreateOrXferBBBBBBBBBBBBBBBB");

#[program]
pub mod create_or_xfer {
    use super::*;
    pub fn init(ctx: Context<InitAx>, do_create_first: bool, unit: u64) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.admin = ctx.accounts.admin.key();
        s.do_create_first = do_create_first;
        s.unit = unit.max(1);
        Ok(())
    }
    pub fn go(ctx: Context<GoAx>) -> Result<()> {
        if ctx.accounts.state.do_create_first {
            associated_token::create(CpiContext::new(
                ctx.accounts.associated_token_program.to_account_info(),
                Create {
                    payer: ctx.accounts.admin.to_account_info(),
                    associated_token: ctx.accounts.ata.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
            ))?;
        }
        if !ctx.accounts.state.do_create_first {
            token::transfer(CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                SplTransfer {
                    from: ctx.accounts.source.to_account_info(),
                    to: ctx.accounts.destination.to_account_info(),
                    authority: ctx.accounts.admin.to_account_info(),
                },
            ), ctx.accounts.state.unit)?;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAx<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 1 + 8)]
    pub state: Account<'info, AxState>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct GoAx<'info> {
    #[account(mut, has_one = admin)]
    pub state: Account<'info, AxState>,
    pub admin: Signer<'info>,
    // create 用
    /// CHECK: Anchor の Create CPI で使用する派生ATA
    #[account(mut)] pub ata: Account<'info, SystemAccount>,
    /// CHECK: オーナー（create の authority）
    pub owner: Signer<'info>,
    /// CHECK: mint（mint自体はカスタム型を用意せず SystemAccount で受ける例）
    pub mint: Account<'info, SystemAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    // transfer 用
    #[account(mut)] pub source: Account<'info, TokenAccount>,
    #[account(mut)] pub destination: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[account] pub struct AxState { pub admin: Pubkey, pub do_create_first: bool, pub unit: u64 }
