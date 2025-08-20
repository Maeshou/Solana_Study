use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Token, TokenAccount, Mint};

declare_id!("PasF06Qy8We3Rt9Uz1Io4Lp6Kd2Mj7Nb5Vc0F006");

#[program]
pub mod season_pass_triangle_v1 {
    use super::*;

    pub fn init_pass(ctx: Context<InitPass>, daily_cap: u64) -> Result<()> {
        let pass_state = &mut ctx.accounts.pass_state;
        pass_state.admin = ctx.accounts.admin.key();
        pass_state.daily_cap = if daily_cap < 5 { 5 } else { daily_cap };
        pass_state.level = 3;
        pass_state.charged_units = daily_cap / 3 + 2;
        pass_state.streak_days = 5;
        Ok(())
    }

    pub fn act_charge(ctx: Context<ActCharge>, base_units: u64, add_days: u16) -> Result<()> {
        let pass_state = &mut ctx.accounts.pass_state;

        // 連続日数更新（上限30）
        let mut days_to_apply: u16 = add_days;
        if pass_state.streak_days as u16 + days_to_apply > 30 {
            let overflow = pass_state.streak_days as u16 + days_to_apply - 30;
            if days_to_apply >= overflow { days_to_apply = days_to_apply - overflow; } else { days_to_apply = 0; }
        }
        pass_state.streak_days = pass_state.streak_days + days_to_apply as u64;

        // 三角数ボーナス
        let triangular_value: u64 = (pass_state.streak_days * (pass_state.streak_days + 1)) / 2;
        let mut grant_units: u64 = base_units + triangular_value / 20 + 1;

        // レベル節目ボーナス（=if を使わず段階判定）
        if pass_state.level % 5 == 0 { grant_units = grant_units + 2; }
        if pass_state.level % 10 == 0 { grant_units = grant_units + 3; }
        if pass_state.level % 15 == 0 { grant_units = grant_units + 4; }

        if grant_units > pass_state.daily_cap {
            grant_units = pass_state.daily_cap;
            pass_state.level = pass_state.level + 1;
        }

        token::mint_to(ctx.accounts.mint_ctx(), grant_units)?;
        pass_state.charged_units = pass_state.charged_units + grant_units;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPass<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 4 + 8 + 8)]
    pub pass_state: Account<'info, PassState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActCharge<'info> {
    #[account(mut, has_one = admin)]
    pub pass_state: Account<'info, PassState>,
    pub admin: Signer<'info>,

    pub pass_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_pass_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActCharge<'info> {
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let c = MintTo {
            mint: self.pass_mint.to_account_info(),
            to: self.user_pass_vault.to_account_info(),
            authority: self.admin.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), c)
    }
}
#[account]
pub struct PassState {
    pub admin: Pubkey,
    pub daily_cap: u64,
    pub level: u32,
    pub charged_units: u64,
    pub streak_days: u64,
}
