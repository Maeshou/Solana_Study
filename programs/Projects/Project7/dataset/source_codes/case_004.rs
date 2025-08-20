use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Token, TokenAccount, Mint};

declare_id!("Cr4ftM4tBuRn11111111111111111111111111111");

#[program]
pub mod craft_material_burn {
    use super::*;
    pub fn init_station(ctx: Context<InitStation>, min_power: u32) -> Result<()> {
        let s = &mut ctx.accounts.station;
        s.owner = ctx.accounts.owner.key();
        s.min_power = min_power;
        s.total_burn = 0;
        s.stage = CraftStage::Idle;
        Ok(())
    }

    pub fn act_craft(ctx: Context<ActCraft>, base_cost: u64, power: u32) -> Result<()> {
        let s = &mut ctx.accounts.station;

        // 分岐とループ（段階的コスト増加）
        let mut cost = base_cost;
        let mut step = 0u32;
        while step < power {
            cost = cost.saturating_add(5);
            step += 1;
        }

        if power < s.min_power {
            s.stage = CraftStage::Cooling;
            return Err(ErrorCode::PowerTooLow.into());
        } else {
            s.stage = CraftStage::Working;
        }

        // CPI: burn (materials from user_materials)
        let cpi_ctx = ctx.accounts.burn_ctx();
        token::burn(cpi_ctx, cost)?;

        s.total_burn = s.total_burn.saturating_add(cost);
        s.stage = CraftStage::Complete;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStation<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 8 + 1)]
    pub station: Account<'info, Station>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActCraft<'info> {
    #[account(mut, has_one = owner)]
    pub station: Account<'info, Station>,
    pub owner: Signer<'info>,

    #[account(mut)]
    pub user_materials: Account<'info, TokenAccount>,
    pub material_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ActCraft<'info> {
    pub fn burn_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let accounts = Burn {
            mint: self.material_mint.to_account_info(),
            from: self.user_materials.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accounts)
    }
}

#[account]
pub struct Station {
    pub owner: Pubkey,
    pub min_power: u32,
    pub total_burn: u64,
    pub stage: CraftStage,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum CraftStage {
    Idle,
    Working,
    Cooling,
    Complete,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Power too low")]
    PowerTooLow,
}
