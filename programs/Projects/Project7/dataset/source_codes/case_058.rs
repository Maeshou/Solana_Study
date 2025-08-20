use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Token, TokenAccount, Mint};

declare_id!("FarmA02YieldM1nT7Lm3R8tD6W4yZ1nC5bK2hU0Q302");

#[program]
pub mod farm_yield_mint_v1 {
    use super::*;

    pub fn init_farm(ctx: Context<InitFarm>, daily_cap_input: u64, base_yield_input: u64) -> Result<()> {
        let farm = &mut ctx.accounts.farm;
        farm.manager = ctx.accounts.manager.key();
        farm.daily_cap = daily_cap_input;
        if farm.daily_cap < 10 { farm.daily_cap = 10; }
        farm.base_yield = base_yield_input;
        if farm.base_yield < 1 { farm.base_yield = 1; }
        farm.emitted_today = 1;
        farm.streak = 1;
        Ok(())
    }

    pub fn act_harvest(ctx: Context<ActHarvest>, plots: u8) -> Result<()> {
        let farm = &mut ctx.accounts.farm;

        // 区画ごと逓増
        let mut total = farm.base_yield;
        let mut i: u8 = 1;
        while i < plots {
            let mut add = farm.base_yield / 6 + 1;
            if i % 3 == 0 { add = add + 2; }
            total = total + add;
            i = i + 1;
        }

        // しきい値で補正
        if farm.streak % 5 == 0 { total = total + total / 10; }
        if farm.streak % 7 == 0 { total = total + 3; }

        let next = farm.emitted_today + total;
        if next > farm.daily_cap {
            let rest = farm.daily_cap - farm.emitted_today;
            if rest > 0 { token::mint_to(ctx.accounts.mint_ctx(), rest)?; }
            farm.emitted_today = farm.daily_cap;
            farm.streak = 1;
            return Err(FarmErr::DailyMax.into());
        }

        token::mint_to(ctx.accounts.mint_ctx(), total)?;
        farm.emitted_today = next;
        farm.streak = farm.streak + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitFarm<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub farm: Account<'info, FarmState>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActHarvest<'info> {
    #[account(mut, has_one = manager)]
    pub farm: Account<'info, FarmState>,
    pub manager: Signer<'info>,

    pub yield_mint: Account<'info, Mint>,
    #[account(mut)]
    pub farmer_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActHarvest<'info> {
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let m = MintTo {
            mint: self.yield_mint.to_account_info(),
            to: self.farmer_vault.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), m)
    }
}

#[account]
pub struct FarmState {
    pub manager: Pubkey,
    pub daily_cap: u64,
    pub base_yield: u64,
    pub emitted_today: u64,
    pub streak: u64,
}
#[error_code]
pub enum FarmErr { #[msg("daily cap reached")] DailyMax }
