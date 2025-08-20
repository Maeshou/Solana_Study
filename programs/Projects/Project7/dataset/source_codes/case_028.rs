use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Token, TokenAccount, Mint};

declare_id!("ForgeV4y1Rk8Mn3Qp7Ls2AaBbCcDdEeFfGgHhIiJj004");

#[program]
pub mod crafting_v4 {
    use super::*;

    pub fn init_station(ctx: Context<InitStation>, required_power: u32, base_progress: u64) -> Result<()> {
        let station = &mut ctx.accounts.station;
        station.creator_key = ctx.accounts.creator.key();
        station.required_power = required_power.max(1);
        station.progress_points = base_progress.max(2);
        station.materials_burned = base_progress.saturating_mul(2);
        station.phase = CraftPhase::Idle;
        Ok(())
    }

    pub fn act_craft(ctx: Context<ActCraft>, provided_power: u32, base_cost: u64, step_count: u8) -> Result<()> {
        let station = &mut ctx.accounts.station;

        // コストはステップ数で逓増、パワーが不足していると中断
        let mut total_cost = base_cost.max(1);
        let mut step_cursor = 0u8;
        while step_cursor < step_count {
            total_cost = total_cost.saturating_add((step_cursor as u64).saturating_add(2));
            step_cursor = step_cursor.saturating_add(1);
        }

        if provided_power < station.required_power {
            station.phase = CraftPhase::Cooling;
            return Err(CraftErr::LowPower.into());
        }
        station.phase = CraftPhase::Heating;

        token::burn(ctx.accounts.burn_ctx(), total_cost)?;
        station.materials_burned = station.materials_burned.saturating_add(total_cost);

        // 進捗はコスト依存で上昇、閾値で段階を切替
        station.progress_points = station.progress_points.saturating_add(total_cost.saturating_div(4).max(1));
        if station.progress_points > station.materials_burned.saturating_div(3) { station.phase = CraftPhase::Assembling; }
        if station.progress_points > station.materials_burned.saturating_sub(station.materials_burned / 5) { station.phase = CraftPhase::Complete; }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStation<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 4 + 8 + 8 + 1)]
    pub station: Account<'info, CraftStation>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActCraft<'info> {
    #[account(mut, has_one = creator_key)]
    pub station: Account<'info, CraftStation>,
    pub creator_key: Signer<'info>,

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
            authority: self.creator_key.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), b)
    }
}

#[account]
pub struct CraftStation {
    pub creator_key: Pubkey,
    pub required_power: u32,
    pub progress_points: u64,
    pub materials_burned: u64,
    pub phase: CraftPhase,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum CraftPhase {
    Idle,
    Heating,
    Assembling,
    Cooling,
    Complete,
}

#[error_code]
pub enum CraftErr {
    #[msg("Provided power is too low")]
    LowPower,
}
