use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, Token, TokenAccount, Mint};

declare_id!("FertHarvest3B7wXk5Qm7Rp9Vs1Lt3Na6Ud8Yh2Zi303");

#[program]
pub mod fertilizer_bonus_harvest_v1 {
    use super::*;

    pub fn init_field(ctx: Context<InitField>, daily_cap_input: u64, base_yield_input: u64) -> Result<()> {
        let field = &mut ctx.accounts.field;
        field.manager = ctx.accounts.manager.key();
        field.daily_cap = daily_cap_input;
        if field.daily_cap < 10 { field.daily_cap = 10; }
        field.base_yield = base_yield_input;
        if field.base_yield < 1 { field.base_yield = 1; }
        field.issued_today = 1;
        field.streak_days = 1;
        Ok(())
    }

    pub fn act_harvest(ctx: Context<ActHarvest>, fertilizer_units: u64, days_since_last: u8) -> Result<()> {
        let field = &mut ctx.accounts.field;

        // 肥料消費
        let mut burn_amount: u64 = fertilizer_units;
        if burn_amount < 1 { burn_amount = 1; }
        token::burn(ctx.accounts.burn_ctx(), burn_amount)?;

        // ストリーク補正（2日ごとに+2%）
        let mut multiplier_percent: u64 = 100;
        let mut day_counter: u8 = 0;
        while day_counter < days_since_last {
            if (day_counter + 1) % 2 == 0 { multiplier_percent = multiplier_percent + 2; }
            day_counter = day_counter + 1;
        }

        let base_units: u64 = field.base_yield + burn_amount / 5 + 1;
        let scaled_units: u64 = (base_units as u128 * multiplier_percent as u128 / 100u128) as u64;

        let projected_total: u64 = field.issued_today + scaled_units;
        if projected_total > field.daily_cap {
            let mint_now = field.daily_cap - field.issued_today;
            if mint_now > 0 { token::mint_to(ctx.accounts.mint_ctx(), mint_now)?; }
            field.issued_today = field.daily_cap;
            field.streak_days = 1;
            return Err(FieldErr::DailyCap.into());
        }

        token::mint_to(ctx.accounts.mint_ctx(), scaled_units)?;
        field.issued_today = projected_total;
        field.streak_days = field.streak_days + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitField<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub field: Account<'info, FieldState>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActHarvest<'info> {
    #[account(mut, has_one = manager)]
    pub field: Account<'info, FieldState>,
    pub manager: Signer<'info>,

    pub fertilizer_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_fertilizer_vault: Account<'info, TokenAccount>,

    pub crop_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_crop_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActHarvest<'info> {
    pub fn burn_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let call = Burn {
            mint: self.fertilizer_mint.to_account_info(),
            from: self.user_fertilizer_vault.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), call)
    }
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let call = MintTo {
            mint: self.crop_mint.to_account_info(),
            to: self.user_crop_vault.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), call)
    }
}
#[account]
pub struct FieldState {
    pub manager: Pubkey,
    pub daily_cap: u64,
    pub base_yield: u64,
    pub issued_today: u64,
    pub streak_days: u64,
}
#[error_code]
pub enum FieldErr { #[msg("daily cap reached")] DailyCap }
