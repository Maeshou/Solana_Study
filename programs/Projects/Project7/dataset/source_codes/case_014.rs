use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Token, TokenAccount, Mint};

declare_id!("ForgeV20000000000000000000000000000000000");

#[program]
pub mod nft_forge_v2 {
    use super::*;

    pub fn init_forge(ctx: Context<InitForge>, min_quality: u16) -> Result<()> {
        let forge = &mut ctx.accounts.forge;
        forge.creator = ctx.accounts.creator.key();
        forge.min_quality = min_quality;
        forge.stage_index = 1;
        forge.materials_consumed = 5; // 開始点を5にして差別化
        forge.state_flag = ForgeState::Dormant;
        Ok(())
    }

    pub fn act_forge(
        ctx: Context<ActForge>,
        base_cost: u64,
        quality: u16,
        steps: u8,
    ) -> Result<()> {
        let forge = &mut ctx.accounts.forge;

        // ループ：ステップごとにコスト上昇
        let mut total_cost = base_cost;
        for _ in 0..steps {
            total_cost = total_cost.saturating_add(3);
        }

        // 分岐：品質に応じた状態遷移
        if quality < forge.min_quality {
            forge.state_flag = ForgeState::Cooling;
            return Err(ForgeErr::LowQuality.into());
        } else {
            forge.state_flag = ForgeState::Heating;
        }

        let cpi = ctx.accounts.burn_materials_ctx();
        token::burn(cpi, total_cost)?;

        forge.materials_consumed = forge.materials_consumed.saturating_add(total_cost);
        forge.stage_index = forge.stage_index.saturating_add(1);
        forge.state_flag = ForgeState::Finished;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitForge<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 2 + 8 + 8 + 1)]
    pub forge: Account<'info, ForgeProfile>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActForge<'info> {
    #[account(mut, has_one = creator)]
    pub forge: Account<'info, ForgeProfile>,
    pub creator: Signer<'info>,

    #[account(mut)]
    pub material_vault: Account<'info, TokenAccount>,
    pub material_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ActForge<'info> {
    pub fn burn_materials_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let accs = Burn {
            mint: self.material_mint.to_account_info(),
            from: self.material_vault.to_account_info(),
            authority: self.creator.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accs)
    }
}

#[account]
pub struct ForgeProfile {
    pub creator: Pubkey,
    pub min_quality: u16,
    pub stage_index: u64,
    pub materials_consumed: u64,
    pub state_flag: ForgeState,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum ForgeState {
    Dormant,
    Heating,
    Cooling,
    Finished,
}

#[error_code]
pub enum ForgeErr {
    #[msg("Quality is too low for forging")]
    LowQuality,
}
