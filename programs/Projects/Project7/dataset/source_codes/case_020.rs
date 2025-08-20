use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Token, TokenAccount, Mint};

declare_id!("PassV3Jq4n6PassV3Jq4n6PassV3Jq4n6PassV3Jq4");

#[program]
pub mod season_pass_v3 {
    use super::*;

    pub fn init_pass(ctx: Context<InitPass>, max_daily: u64, base_lvl: u32) -> Result<()> {
        let p = &mut ctx.accounts.pass;
        p.admin = ctx.accounts.admin.key();
        p.max_daily_grant = max_daily.max(5);
        p.current_level = base_lvl.max(2);
        p.total_charged = max_daily.saturating_div(2).max(3);
        p.streak_days = 3;
        Ok(())
    }

    pub fn act_charge(ctx: Context<ActCharge>, base_units: u64, streak_increment: u16) -> Result<()> {
        let p = &mut ctx.accounts.pass;

        // 連続日数を増加（上限30）
        let mut i = 0u16;
        while i < streak_increment {
            if p.streak_days < 30 { p.streak_days = p.streak_days.saturating_add(1); }
            i = i.saturating_add(1);
        }

        // 付与量：ベース + 連続ボーナスの平方根近似（整数）
        let mut bonus = 1u64;
        let mut s = 1u64;
        while s.saturating_mul(s) <= p.streak_days as u64 {
            bonus = bonus.saturating_add(1);
            s = s.saturating_add(1);
        }
        let mut grant = base_units.saturating_add(bonus);

        if grant > p.max_daily_grant {
            grant = p.max_daily_grant;
            p.current_level = p.current_level.saturating_add(1);
        }

        token::mint_to(ctx.accounts.mint_ctx(), grant)?;
        p.total_charged = p.total_charged.saturating_add(grant);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPass<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 4 + 8 + 8)]
    pub pass: Account<'info, PassState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActCharge<'info> {
    #[account(mut, has_one = admin)]
    pub pass: Account<'info, PassState>,
    pub admin: Signer<'info>,
    pub pass_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_pass_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ActCharge<'info> {
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let a = MintTo {
            mint: self.pass_mint.to_account_info(),
            to: self.user_pass_vault.to_account_info(),
            authority: self.admin.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
}

#[account]
pub struct PassState {
    pub admin: Pubkey,
    pub max_daily_grant: u64,
    pub current_level: u32,
    pub total_charged: u64,
    pub streak_days: u64,
}
