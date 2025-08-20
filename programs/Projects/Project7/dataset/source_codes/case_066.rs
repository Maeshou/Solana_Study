use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("DropA10LadderM3tN7Lm3R8tD6W4yZ1nC5bK2hU0Y310");

#[program]
pub mod airdrop_ladder_v1 {
    use super::*;

    pub fn init_drop(ctx: Context<InitDrop>, base_units_input: u64, cap_per_day_input: u64) -> Result<()> {
        let drop = &mut ctx.accounts.drop;
        drop.operator = ctx.accounts.operator.key();
        drop.base_units = base_units_input;
        if drop.base_units < 1 { drop.base_units = 1; }
        drop.cap_per_day = cap_per_day_input;
        if drop.cap_per_day < drop.base_units { drop.cap_per_day = drop.base_units; }
        drop.issued_today = 0;
        drop.first_movers = 0;
        Ok(())
    }

    pub fn act_claim(ctx: Context<ActClaim>, ladder_steps: u8, is_early: bool) -> Result<()> {
        let drop = &mut ctx.accounts.drop;

        // 段階要求：ステップごとに+1,+2,+3...
        let mut grant = drop.base_units;
        let mut s: u8 = 0;
        while s < ladder_steps {
            grant = grant + (s as u64 + 1);
            s = s + 1;
        }

        // 先着優遇
        if is_early {
            grant = grant + 2;
            drop.first_movers = drop.first_movers + 1;
        }

        let projected = drop.issued_today + grant;
        if projected > drop.cap_per_day {
            let rest = drop.cap_per_day - drop.issued_today;
            token::transfer(ctx.accounts.pool_to_user(), rest)?;
            drop.issued_today = drop.cap_per_day;
            return Err(DropErr::Cap.into());
        }

        token::transfer(ctx.accounts.pool_to_user(), grant)?;
        drop.issued_today = projected;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitDrop<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub drop: Account<'info, DropState>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActClaim<'info> {
    #[account(mut, has_one = operator)]
    pub drop: Account<'info, DropState>,
    pub operator: Signer<'info>,

    #[account(mut)]
    pub drop_pool_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActClaim<'info> {
    pub fn pool_to_user(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from:self.drop_pool_vault.to_account_info(), to:self.user_vault.to_account_info(), authority:self.operator.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
}
#[account]
pub struct DropState {
    pub operator: Pubkey,
    pub base_units: u64,
    pub cap_per_day: u64,
    pub issued_today: u64,
    pub first_movers: u64,
}
#[error_code]
pub enum DropErr { #[msg("cap per day reached")] Cap }
