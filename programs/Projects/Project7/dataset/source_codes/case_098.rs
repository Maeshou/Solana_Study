// 07. エネルギー回復トークン配布
use anchor_lang::prelude::*;
use anchor_spl::token::*;

declare_id!("6hB5F4e3D2c1B0a9v8U7t6S5r4Q3p2O1n0M9l8K7j6I5H4g3F2E1D0C9B8A7f6");

#[program]
pub mod energy_booster {
    use super::*;

    pub fn initialize_booster(ctx: Context<InitializeBooster>, daily_limit: u32) -> Result<()> {
        let booster = &mut ctx.accounts.booster_state;
        booster.admin = ctx.accounts.admin.key();
        booster.daily_limit = daily_limit;
        booster.current_day = Clock::get()?.unix_timestamp / 86400;
        booster.redeem_token_mint = ctx.accounts.redeem_token_mint.key();
        booster.redeem_token_amount = 100;
        Ok(())
    }

    pub fn redeem_energy(ctx: Context<RedeemEnergy>) -> Result<()> {
        let booster = &mut ctx.accounts.booster_state;
        let mut user_state = ctx.accounts.user_state.load_mut()?;
        let current_day = Clock::get()?.unix_timestamp / 86400;

        // Reset if new day
        if user_state.last_redeem_day != current_day {
            user_state.redeems_today = 0;
            user_state.last_redeem_day = current_day;
        }

        if user_state.redeems_today >= booster.daily_limit {
            return Err(ErrorCode::DailyLimitReached.into());
        }

        let mut remaining_boosts = booster.daily_limit - user_state.redeems_today;

        while remaining_boosts > 0 {
            if user_state.redeems_today < booster.daily_limit {
                let cpi_accounts = MintTo {
                    mint: ctx.accounts.redeem_token_mint.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.admin.to_account_info(),
                };
                let cpi_program = ctx.accounts.token_program.to_account_info();
                let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
                token::mint_to(cpi_ctx, booster.redeem_token_amount)?;
                user_state.redeems_today += 1;
                remaining_boosts = 0; // Only one boost per call for this example
            } else {
                break;
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(daily_limit: u32)]
pub struct InitializeBooster<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 4 + 8 + 32 + 8)]
    pub booster_state: Account<'info, EnergyBoosterState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub redeem_token_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RedeemEnergy<'info> {
    #[account(has_one = admin, has_one = redeem_token_mint)]
    pub booster_state: Account<'info, EnergyBoosterState>,
    #[account(mut)]
    pub redeem_token_mint: Account<'info, Mint>,
    #[account(mut, has_one = owner)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(init_if_needed, payer = owner, space = 8 + 8 + 4, seeds = [b"user_energy", owner.key().as_ref()], bump)]
    pub user_state: AccountLoader<'info, UserEnergyState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: Admin authority for minting, checked via booster_state
    pub admin: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct EnergyBoosterState {
    pub admin: Pubkey,
    pub daily_limit: u32,
    pub current_day: i64,
    pub redeem_token_mint: Pubkey,
    pub redeem_token_amount: u64,
}

#[account(zero_copy)]
pub struct UserEnergyState {
    pub last_redeem_day: i64,
    pub redeems_today: u32,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Daily redemption limit has been reached.")]
    DailyLimitReached,
}