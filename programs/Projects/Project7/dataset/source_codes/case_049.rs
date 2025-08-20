use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Token, TokenAccount, Mint};

declare_id!("Bo0S3terGrantX8R1M5Q2L9C7V4N6Y3P0T5Z303");

#[program]
pub mod booster_grant_v1 {
    use super::*;

    pub fn init_station(ctx: Context<InitStation>, daily_cap: u64, base_grant: u64) -> Result<()> {
        let station = &mut ctx.accounts.station;
        station.admin = ctx.accounts.admin.key();
        station.daily_cap = if daily_cap < 10 { 10 } else { daily_cap };
        station.base_grant = if base_grant < 1 { 1 } else { base_grant };
        station.issued_today = base_grant + 2;
        station.streak_counter = 3;
        Ok(())
    }

    pub fn act_grant(ctx: Context<ActGrant>, actions: u8) -> Result<()> {
        let station = &mut ctx.accounts.station;

        // 行動回数に応じて増分
        let mut bonus_units = 0u64;
        let mut i: u8 = 0;
        while i < actions {
            bonus_units = bonus_units + ((i as u64 % 4) + 1);
            i = i + 1;
        }

        // ストリークによる段階ボーナス
        let mut grant = station.base_grant + bonus_units;
        if station.streak_counter % 3 == 0 { grant = grant + 2; }
        if station.streak_counter % 5 == 0 { grant = grant + 3; }

        let projected = station.issued_today + grant;
        if projected > station.daily_cap {
            let remaining = station.daily_cap - station.issued_today;
            token::mint_to(ctx.accounts.mint_ctx(), remaining)?;
            station.issued_today = station.daily_cap;
            station.streak_counter = 1;
            return Err(BoosterErr::DailyMax.into());
        }

        token::mint_to(ctx.accounts.mint_ctx(), grant)?;
        station.issued_today = projected;
        station.streak_counter = station.streak_counter + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStation<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub station: Account<'info, BoosterStation>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActGrant<'info> {
    #[account(mut, has_one = admin)]
    pub station: Account<'info, BoosterStation>,
    pub admin: Signer<'info>,

    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub recipient_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActGrant<'info> {
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let m = MintTo {
            mint: self.mint.to_account_info(),
            to: self.recipient_vault.to_account_info(),
            authority: self.admin.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), m)
    }
}

#[account]
pub struct BoosterStation {
    pub admin: Pubkey,
    pub daily_cap: u64,
    pub base_grant: u64,
    pub issued_today: u64,
    pub streak_counter: u64,
}

#[error_code]
pub enum BoosterErr {
    #[msg("daily limit reached")]
    DailyMax,
}
