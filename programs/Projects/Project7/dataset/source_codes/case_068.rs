use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, Token, TokenAccount, Mint};

declare_id!("PassExt2KeyB8uZk3Xn5Rc7Vt9Lm1Qa4Ws6Ed8Ty302");

#[program]
pub mod pass_extension_craft_v1 {
    use super::*;

    pub fn init_station(ctx: Context<InitStation>, base_key_cost_input: u64) -> Result<()> {
        let station = &mut ctx.accounts.station;
        station.owner = ctx.accounts.owner.key();
        station.base_key_cost = base_key_cost_input;
        if station.base_key_cost < 1 { station.base_key_cost = 1; }
        station.tier = 1;
        station.total_keys_burned = 1;
        station.total_passes_minted = 0;
        Ok(())
    }

    pub fn act_extend(ctx: Context<ActExtend>, requested_days: u16, layers_to_apply: u8) -> Result<()> {
        let station = &mut ctx.accounts.station;

        // 段階コスト：層ごとに増し
        let mut burn_units: u64 = station.base_key_cost;
        let mut layer_counter: u8 = 0;
        while layer_counter < layers_to_apply {
            burn_units = burn_units + burn_units / 5 + 1;
            layer_counter = layer_counter + 1;
        }

        // 期間係数
        let mut pass_units: u64 = requested_days as u64 / 3 + 1;
        if station.tier >= 3 { pass_units = pass_units + 2; }
        if station.tier >= 5 { pass_units = pass_units + 3; }

        token::burn(ctx.accounts.burn_ctx(), burn_units)?;
        token::mint_to(ctx.accounts.mint_ctx(), pass_units)?;

        station.total_keys_burned = station.total_keys_burned + burn_units;
        station.total_passes_minted = station.total_passes_minted + pass_units;
        station.tier = station.tier + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStation<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 4 + 8 + 8)]
    pub station: Account<'info, PassStation>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActExtend<'info> {
    #[account(mut, has_one = owner)]
    pub station: Account<'info, PassStation>,
    pub owner: Signer<'info>,

    pub key_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_key_vault: Account<'info, TokenAccount>,

    pub pass_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_pass_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActExtend<'info> {
    pub fn burn_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let call = Burn {
            mint: self.key_mint.to_account_info(),
            from: self.user_key_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), call)
    }
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let call = MintTo {
            mint: self.pass_mint.to_account_info(),
            to: self.user_pass_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), call)
    }
}
#[account]
pub struct PassStation {
    pub owner: Pubkey,
    pub base_key_cost: u64,
    pub tier: u32,
    pub total_keys_burned: u64,
    pub total_passes_minted: u64,
}
