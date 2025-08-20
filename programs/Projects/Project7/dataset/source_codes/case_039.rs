use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Token, TokenAccount, Mint};

declare_id!("CrfD04Uz6Ma8Qp1Ls9Vk3Wt5Bn2Yh7Re4Tg0D004");

#[program]
pub mod crafting_exponential_v1 {
    use super::*;

    pub fn init_station(ctx: Context<InitStation>, required_power: u32) -> Result<()> {
        let station_state = &mut ctx.accounts.station_state;
        station_state.owner = ctx.accounts.owner.key();
        station_state.required_power = if required_power < 1 { 1 } else { required_power };
        station_state.progress_points = 5;
        station_state.materials_burned = 7;
        station_state.cooldown_counter = 2;
        Ok(())
    }

    pub fn act_craft(ctx: Context<ActCraft>, provided_power: u32, base_cost: u64, layer_count: u8) -> Result<()> {
        let station_state = &mut ctx.accounts.station_state;

        // 近似指数コスト：毎層で cost = cost + cost/6 + 1
        let mut total_cost: u64 = if base_cost < 1 { 1 } else { base_cost };
        let mut layer_cursor: u8 = 0;
        while layer_cursor < layer_count {
            total_cost = total_cost + total_cost / 6 + 1;
            layer_cursor = layer_cursor + 1;
        }

        // クールタイム消化
        if station_state.cooldown_counter > 0 {
            station_state.cooldown_counter = station_state.cooldown_counter - 1;
        }

        if provided_power < station_state.required_power {
            station_state.cooldown_counter = station_state.cooldown_counter + 2;
            return Err(CraftErr::PowerLow.into());
        }

        token::burn(ctx.accounts.burn_ctx(), total_cost)?;
        station_state.materials_burned = station_state.materials_burned + total_cost;
        station_state.progress_points = station_state.progress_points + (total_cost / 5 + 1);

        if station_state.progress_points > station_state.materials_burned / 3 {
            station_state.cooldown_counter = station_state.cooldown_counter + 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStation<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 8 + 8 + 8)]
    pub station_state: Account<'info, StationState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActCraft<'info> {
    #[account(mut, has_one = owner)]
    pub station_state: Account<'info, StationState>,
    pub owner: Signer<'info>,

    #[account(mut)]
    pub material_vault: Account<'info, TokenAccount>,
    pub material_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActCraft<'info> {
    pub fn burn_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let b = Burn {
            mint: self.material_mint.to_account_info(),
            from: self.material_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), b)
    }
}
#[account]
pub struct StationState {
    pub owner: Pubkey,
    pub required_power: u32,
    pub progress_points: u64,
    pub materials_burned: u64,
    pub cooldown_counter: u64,
}
#[error_code]
pub enum CraftErr {
    #[msg("required power not met")]
    PowerLow,
}
