use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, Token, TokenAccount, Mint};

declare_id!("Carb4nRetir3Hj7Qm2Lx5Zp8Vt1Na4Ur6Hs9Kd904");

#[program]
pub mod carbon_retirement_badge_v1 {
    use super::*;

    pub fn init_registry(ctx: Context<InitRegistry>, base_burn_input: u64) -> Result<()> {
        let r = &mut ctx.accounts.registry;
        r.authority = ctx.accounts.authority.key();
        r.base_burn = base_burn_input;
        if r.base_burn < 1 { r.base_burn = 1; }
        r.total_burned = 0;
        r.badges_issued = 0;
        Ok(())
    }

    pub fn act_retire(ctx: Context<ActRetire>, burn_multiplier: u8, sustain_days: u16) -> Result<()> {
        let r = &mut ctx.accounts.registry;

        let mut burn_units = r.base_burn;
        let mut k: u8 = 0;
        while k < burn_multiplier {
            burn_units = burn_units + r.base_burn / 4 + 1;
            k = k + 1;
        }
        token::burn(ctx.accounts.burn_ctx(), burn_units)?;
        r.total_burned = r.total_burned + burn_units;

        let mut reward = burn_units / 10 + 1;
        let mut d: u16 = 0;
        while d < sustain_days {
            if (d + 1) % 30 == 0 { reward = reward + 2; }
            d = d + 1;
        }
        token::mint_to(ctx.accounts.mint_ctx(), reward)?;
        r.badges_issued = r.badges_issued + reward;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRegistry<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8 + 8 + 8)]
    pub registry: Account<'info, CarbonRegistry>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActRetire<'info> {
    #[account(mut, has_one = authority)]
    pub registry: Account<'info, CarbonRegistry>,
    pub authority: Signer<'info>,

    pub credit_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_credit_vault: Account<'info, TokenAccount>,

    pub badge_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_badge_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActRetire<'info> {
    pub fn burn_ctx(&self)->CpiContext<'_, '_, '_, 'info, Burn<'info>>{
        let b=Burn{mint:self.credit_mint.to_account_info(),from:self.user_credit_vault.to_account_info(),authority:self.authority.to_account_info()};
        CpiContext::new(self.token_program.to_account_info(),b)
    }
    pub fn mint_ctx(&self)->CpiContext<'_, '_, '_, 'info, MintTo<'info>>{
        let m=MintTo{mint:self.badge_mint.to_account_info(),to:self.user_badge_vault.to_account_info(),authority:self.authority.to_account_info()};
        CpiContext::new(self.token_program.to_account_info(),m)
    }
}
#[account]
pub struct CarbonRegistry {
    pub authority: Pubkey,
    pub base_burn: u64,
    pub total_burned: u64,
    pub badges_issued: u64,
}
