use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Token, TokenAccount, Mint};

declare_id!("ForgeV3A6pL0m5A6pL0m5A6pL0m5A6pL0m5A6pL0m5");

#[program]
pub mod forging_station_v3 {
    use super::*;

    pub fn init_station(ctx: Context<InitStation>, min_power: u32, base_step: u64) -> Result<()> {
        let s = &mut ctx.accounts.station;
        s.creator = ctx.accounts.creator.key();
        s.required_power = min_power.max(1);
        s.progress = base_step.max(2);            // 0では始めない
        s.total_burned = base_step.max(2);
        s.phase = CraftPhase::Idle;
        Ok(())
    }

    pub fn act_craft(ctx: Context<ActCraft>, power: u32, base_cost: u64) -> Result<()> {
        let s = &mut ctx.accounts.station;

        // 段階的コスト（powerが高いほど増加）
        let mut cost = base_cost.max(1);
        let mut t = 0u32;
        while t < power {
            cost = cost.saturating_add((t as u64 % 7).saturating_add(1));
            t = t.saturating_add(1);
        }

        if power < s.required_power {
            s.phase = CraftPhase::Cooling;
            return Err(ForgeErr::NotEnoughPower.into());
        }
        s.phase = CraftPhase::Heating;

        token::burn(ctx.accounts.burn_ctx(), cost)?;

        // 進捗ゲージ更新（一定しきい値でフェーズ遷移）
        s.progress = s.progress.saturating_add(cost.saturating_div(3).max(1));
        s.total_burned = s.total_burned.saturating_add(cost);
        if s.progress > s.total_burned.saturating_div(2) { s.phase = CraftPhase::Assembling; }
        if s.progress > s.total_burned.saturating_sub(s.total_burned / 4) { s.phase = CraftPhase::Complete; }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStation<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 4 + 8 + 8 + 1)]
    pub station: Account<'info, Station>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActCraft<'info> {
    #[account(mut, has_one = creator)]
    pub station: Account<'info, Station>,
    pub creator: Signer<'info>,
    #[account(mut)]
    pub material_vault: Account<'info, TokenAccount>,
    pub material_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ActCraft<'info> {
    pub fn burn_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let accs = Burn {
            mint: self.material_mint.to_account_info(),
            from: self.material_vault.to_account_info(),
            authority: self.creator.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accs)
    }
}

#[account]
pub struct Station {
    pub creator: Pubkey,
    pub required_power: u32,
    pub progress: u64,
    pub total_burned: u64,
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
pub enum ForgeErr {
    #[msg("Insufficient power")]
    NotEnoughPower,
}
