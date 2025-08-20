use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("Ra1dRew4rdD1str1bUt0r11111111111111111111");

#[program]
pub mod raid_distribution {
    use super::*;
    pub fn init_boss(ctx: Context<InitBoss>, base_drop: u64, bonus_step: u8) -> Result<()> {
        let b = &mut ctx.accounts.boss;
        b.admin = ctx.accounts.admin.key();
        b.base_drop = base_drop;
        b.bonus_step = bonus_step;
        b.total_dropped = 0;
        b.hard_mode = false;
        Ok(())
    }

    pub fn act_drop(ctx: Context<ActDrop>, combo: u32, participants: u8) -> Result<()> {
        let b = &mut ctx.accounts.boss;
        require!(participants > 0, ErrorCode::NoParticipants);

        let mut bonus = 0u64;
        for _ in 0..participants {
            bonus = bonus.saturating_add((b.bonus_step as u64) * 10);
        }

        let mut reward = b.base_drop.saturating_add(bonus);
        if combo >= 50 {
            b.hard_mode = true;
            reward = reward.saturating_mul(2);
        } else {
            b.hard_mode = false;
        }

        // CPI: transfer (reward_vault -> player_vault)
        let cpi_ctx = ctx.accounts.transfer_ctx();
        token::transfer(cpi_ctx, reward)?;

        b.total_dropped = b.total_dropped.saturating_add(reward);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBoss<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 1 + 8 + 1)]
    pub boss: Account<'info, Boss>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActDrop<'info> {
    #[account(mut, has_one = admin)]
    pub boss: Account<'info, Boss>,
    pub admin: Signer<'info>,

    #[account(mut)]
    pub reward_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub player_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ActDrop<'info> {
    pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let accounts = Transfer {
            from: self.reward_vault.to_account_info(),
            to: self.player_vault.to_account_info(),
            authority: self.admin.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accounts)
    }
}

#[account]
pub struct Boss {
    pub admin: Pubkey,
    pub base_drop: u64,
    pub bonus_step: u8,
    pub total_dropped: u64,
    pub hard_mode: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("No participants")]
    NoParticipants,
}
