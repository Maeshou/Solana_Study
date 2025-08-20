use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Token, TokenAccount, Mint};

declare_id!("PassV4m3Jn8Qw4Rt7Ux2AaBbCcDdEeFfGgHhIiJj006");

#[program]
pub mod season_pass_v4 {
    use super::*;

    pub fn init_pass(ctx: Context<InitPass>, daily_cap: u64, base_level: u32) -> Result<()> {
        let pass = &mut ctx.accounts.pass;
        pass.admin_key = ctx.accounts.admin.key();
        pass.daily_cap = daily_cap.max(5);
        pass.current_level = base_level.max(2);
        pass.total_units_charged = daily_cap.saturating_div(3).max(3);
        pass.day_streak = 4;
        Ok(())
    }

    pub fn act_charge(ctx: Context<ActCharge>, base_units: u64, extra_days: u16) -> Result<()> {
        let pass = &mut ctx.accounts.pass;

        // 連続日数の更新（上限30）
        let mut cursor = 0u16;
        while cursor < extra_days {
            if pass.day_streak < 30 { pass.day_streak = pass.day_streak.saturating_add(1); }
            cursor = cursor.saturating_add(1);
        }

        // 付与量：ベース＋区切りボーナス（5日と10日）
        let mut grant_units = base_units.max(1);
        if pass.day_streak % 5 == 0 { grant_units = grant_units.saturating_add(2); }
        if pass.day_streak % 10 == 0 { grant_units = grant_units.saturating_add(3); }

        if grant_units > pass.daily_cap {
            grant_units = pass.daily_cap;
            pass.current_level = pass.current_level.saturating_add(1);
        }

        token::mint_to(ctx.accounts.mint_ctx(), grant_units)?;
        pass.total_units_charged = pass.total_units_charged.saturating_add(grant_units);
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
    #[account(mut, has_one = admin_key)]
    pub pass: Account<'info, PassState>,
    pub admin_key: Signer<'info>,

    pub pass_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_pass_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ActCharge<'info> {
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let m = MintTo {
            mint: self.pass_mint.to_account_info(),
            to: self.user_pass_vault.to_account_info(),
            authority: self.admin_key.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), m)
    }
}

#[account]
pub struct PassState {
    pub admin_key: Pubkey,
    pub daily_cap: u64,
    pub current_level: u32,
    pub total_units_charged: u64,
    pub day_streak: u64,
}
