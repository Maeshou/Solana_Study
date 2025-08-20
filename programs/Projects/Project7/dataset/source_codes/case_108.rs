// 8) カウンタの剰余で Token/Token2022/AssociatedToken を順番に呼ぶ（if を3つ）
use anchor_lang::prelude::*;
use anchor_spl::associated_token::{self, AssociatedToken, Create};
use anchor_spl::token::{self, Transfer as Xfer, Token, TokenAccount};
use anchor_spl::token_2022::{self as token_2022, Transfer as Xfer22, Token2022};

declare_id!("AcctOnlyRoundRobinTTATT2022GGGGGGGGGGGGGG");

#[program]
pub mod tri_route {
    use super::*;
    pub fn init(ctx: Context<InitTri>, unit: u64) -> Result<()> {
        let r = &mut ctx.accounts.route;
        r.owner = ctx.accounts.owner.key();
        r.unit = unit.max(1);
        r.counter = 0;
        Ok(())
    }
    pub fn exec(ctx: Context<ExecTri>) -> Result<()> {
        let r = &mut ctx.accounts.route;
        let m = r.counter % 3;

        if m == 0 {
            token::transfer(CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Xfer {
                    from: ctx.accounts.a_legacy.to_account_info(),
                    to: ctx.accounts.b_legacy.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ), r.unit)?;
        }
        if m == 1 {
            token_2022::transfer(CpiContext::new(
                ctx.accounts.token2022_program.to_account_info(),
                Xfer22 {
                    from: ctx.accounts.a22.to_account_info(),
                    to: ctx.accounts.b22.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ), r.unit)?;
        }
        if m == 2 {
            associated_token::create(CpiContext::new(
                ctx.accounts.associated_token_program.to_account_info(),
                Create {
                    payer: ctx.accounts.owner.to_account_info(),
                    associated_token: ctx.accounts.any_ata.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
            ))?;
        }
        r.counter = r.counter.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTri<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8)]
    pub route: Account<'info, TriRoute>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ExecTri<'info> {
    #[account(mut, has_one = owner)]
    pub route: Account<'info, TriRoute>,
    pub owner: Signer<'info>,
    // legacy
    #[account(mut)] pub a_legacy: Account<'info, TokenAccount>,
    #[account(mut)] pub b_legacy: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    // 2022
    #[account(mut)] pub a22: Account<'info, token_2022::TokenAccount>,
    #[account(mut)] pub b22: Account<'info, token_2022::TokenAccount>,
    pub token2022_program: Program<'info, Token2022>,
    // associated token create
    /// CHECK: ATA作成先
    #[account(mut)] pub any_ata: Account<'info, SystemAccount>,
    /// CHECK: 該当 mint
    pub mint: Account<'info, SystemAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
#[account] pub struct TriRoute { pub owner: Pubkey, pub unit: u64, pub counter: u64 }
