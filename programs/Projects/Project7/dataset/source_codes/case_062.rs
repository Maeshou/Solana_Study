use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, Token, TokenAccount, Mint};

declare_id!("BadgA06EvoL7vN7Lm3R8tD6W4yZ1nC5bK2hU0U306");

#[program]
pub mod badge_evolution_v1 {
    use super::*;

    pub fn init_profile(ctx: Context<InitProfile>, evolution_points_input: u64) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        profile.owner = ctx.accounts.owner.key();
        profile.evolution_points = evolution_points_input;
        if profile.evolution_points < 1 { profile.evolution_points = 1; }
        profile.evolved = 0;
        profile.stage = 1;
        Ok(())
    }

    pub fn act_evolve(ctx: Context<ActEvolve>, burn_units_input: u64) -> Result<()> {
        let profile = &mut ctx.accounts.profile;

        let mut burn_units = burn_units_input;
        if burn_units < 1 { burn_units = 1; }

        // 進化点で増幅
        let mut amplify = burn_units / 4 + 1;
        let mut t: u8 = 0;
        while t < (profile.stage % 5) as u8 {
            amplify = amplify + 1;
            t = t + 1;
        }
        let mint_units = burn_units + amplify;

        token::burn(ctx.accounts.burn_ctx(), burn_units)?;
        token::mint_to(ctx.accounts.mint_ctx(), mint_units)?;

        profile.evolved = profile.evolved + 1;
        profile.stage = profile.stage + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitProfile<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8)]
    pub profile: Account<'info, ProfileState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActEvolve<'info> {
    #[account(mut, has_one = owner)]
    pub profile: Account<'info, ProfileState>,
    pub owner: Signer<'info>,

    pub old_badge_mint: Account<'info, Mint>,
    #[account(mut)]
    pub old_badge_vault: Account<'info, TokenAccount>,

    pub new_badge_mint: Account<'info, Mint>,
    #[account(mut)]
    pub new_badge_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActEvolve<'info> {
    pub fn burn_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let b = Burn {
            mint: self.old_badge_mint.to_account_info(),
            from: self.old_badge_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), b)
    }
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let m = MintTo {
            mint: self.new_badge_mint.to_account_info(),
            to: self.new_badge_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), m)
    }
}
#[account]
pub struct ProfileState {
    pub owner: Pubkey,
    pub evolution_points: u64,
    pub evolved: u64,
    pub stage: u64,
}
