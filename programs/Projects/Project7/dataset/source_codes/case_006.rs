use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Token, TokenAccount, Mint};

declare_id!("Se4sonP4ssCh4rg3r111111111111111111111111");

#[program]
pub mod season_pass_charge {
    use super::*;
    pub fn init_pass(ctx: Context<InitPass>, max_bonus: u32) -> Result<()> {
        let p = &mut ctx.accounts.pass;
        p.admin = ctx.accounts.admin.key();
        p.max_bonus = max_bonus;
        p.charged = 0;
        p.level = 1;
        Ok(())
    }

    pub fn act_charge(ctx: Context<ActCharge>, base_units: u64, streak_days: u16) -> Result<()> {
        let p = &mut ctx.accounts.pass;
        let mut bonus = 0u64;
        for _ in 0..streak_days.min(30) {
            bonus = bonus.saturating_add(1);
        }
        let mut amount = base_units.saturating_add(bonus);

        if amount as u32 > p.max_bonus {
            amount = p.max_bonus as u64;
            p.level = p.level.saturating_add(1);
        } else {
            p.level = p.level.saturating_add(0);
        }

        // CPI: mint_to (mint -> user_pass_vault)
        let cpi_ctx = ctx.accounts.mint_to_ctx();
        token::mint_to(cpi_ctx, amount)?;
        p.charged = p.charged.saturating_add(amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPass<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 4 + 8 + 4)]
    pub pass: Account<'info, PassState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActCharge<'info> {
    #[account(mut, has_one = admin)]
    pub pass: Account<'info, PassState>,
    pub admin: Signer<'info>,

    pub pass_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_pass_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ActCharge<'info> {
    pub fn mint_to_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let accs = MintTo {
            mint: self.pass_mint.to_account_info(),
            to: self.user_pass_vault.to_account_info(),
            authority: self.admin.to_account_info(), // admin が mint authority
        };
        CpiContext::new(self.token_program.to_account_info(), accs)
    }
}

#[account]
pub struct PassState {
    pub admin: Pubkey,
    pub max_bonus: u32,
    pub charged: u64,
    pub level: u32,
}
