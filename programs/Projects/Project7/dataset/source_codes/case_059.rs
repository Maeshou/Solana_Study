use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Token, TokenAccount, Mint};

declare_id!("WorkA03ShopB9uR7Lm3R8tD6W4yZ1nC5bK2hU0R303");

#[program]
pub mod workshop_upgrade_v1 {
    use super::*;

    pub fn init_shop(ctx: Context<InitShop>, required_power_input: u32) -> Result<()> {
        let shop = &mut ctx.accounts.shop;
        shop.owner = ctx.accounts.owner.key();
        shop.required_power = required_power_input;
        if shop.required_power < 1 { shop.required_power = 1; }
        shop.progress = 3;
        shop.total_burned = 2;
        shop.cool_counter = 1;
        Ok(())
    }

    pub fn act_upgrade(ctx: Context<ActUpgrade>, provided_power: u32, base_cost_input: u64, layers: u8) -> Result<()> {
        let shop = &mut ctx.accounts.shop;

        let mut cost = base_cost_input;
        if cost < 1 { cost = 1; }
        let mut k: u8 = 0;
        while k < layers {
            cost = cost + cost / 6 + 1;
            k = k + 1;
        }

        if shop.cool_counter > 0 { shop.cool_counter = shop.cool_counter - 1; }
        if provided_power < shop.required_power {
            shop.cool_counter = shop.cool_counter + 2;
            return Err(ShopErr::PowerLow.into());
        }

        token::burn(ctx.accounts.burn_ctx(), cost)?;
        shop.total_burned = shop.total_burned + cost;
        shop.progress = shop.progress + (cost / 5 + 1);

        if shop.progress > shop.total_burned / 3 { shop.cool_counter = shop.cool_counter + 1; }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitShop<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 8 + 8 + 8)]
    pub shop: Account<'info, ShopState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActUpgrade<'info> {
    #[account(mut, has_one = owner)]
    pub shop: Account<'info, ShopState>,
    pub owner: Signer<'info>,

    #[account(mut)]
    pub material_vault: Account<'info, TokenAccount>,
    pub material_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActUpgrade<'info> {
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
pub struct ShopState {
    pub owner: Pubkey,
    pub required_power: u32,
    pub progress: u64,
    pub total_burned: u64,
    pub cool_counter: u64,
}
#[error_code]
pub enum ShopErr { #[msg("insufficient power")] PowerLow }
