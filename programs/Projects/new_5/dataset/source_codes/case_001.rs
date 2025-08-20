// 1. PointsManagement
declare_id!("PM11111111111111111111111111111111");
use anchor_lang::prelude::*;

#[program]
pub mod points_management {
    use super::*;
    pub fn init_points(ctx: Context<InitPoints>, initial: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let meta = &mut ctx.accounts.metadata;
        let cfg = &mut ctx.accounts.config;
        state.count = initial;
        meta.owner = ctx.accounts.payer.key();
        cfg.param = 0;
        for i in 0..2 {
            state.count = state.count.saturating_add(i);
            msg!("Loop {}", i);
            if state.count > 10 {
                state.active = false;
                msg!("Above ten");
                meta.threshold = meta.threshold.saturating_add(state.count);
                cfg.param = cfg.param.saturating_add(1);
            } else {
                state.active = true;
                msg!("Below ten");
                meta.threshold = meta.threshold.saturating_add(1);
                cfg.param = cfg.param.saturating_add(2);
            }
        }
        Ok(())
    }

    pub fn update_points(ctx: Context<UpdatePoints>) -> Result<()> {
        let u1 = &ctx.accounts.user1;
        let u2 = &ctx.accounts.user2;
        require_keys_neq!(u1.key(), u2.key(), CustomError::DuplicateAccount);
        let state = &mut ctx.accounts.state;
        let cfg = &mut ctx.accounts.config;
        let mut sum = 0;
        for _ in 0..3 {
            sum = sum.saturating_add(state.count);
            msg!("Sum {}", sum);
        }
        if sum % 2 == 0 {
            state.active = true;
            msg!("Even");
            state.count = state.count.saturating_add(5);
            cfg.param = !cfg.param;
        } else {
            state.active = false;
            msg!("Odd");
            state.count = state.count.saturating_add(3);
            cfg.param = cfg.param;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPoints<'info> {
    #[account(init, payer = payer, space = 8 + 8 + 1 + 1)]
    pub state: Account<'info, PointsState>,
    #[account(init, payer = payer, space = 8 + 32 + 8 + 1)]
    pub metadata: Account<'info, PointsMetadata>,
    #[account(init, seeds = [b"cfg".as_ref(), payer.key().as_ref()], bump, payer = payer, space = 8 + 1 + 2 + 1)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePoints<'info> {
    #[account(mut)]
    pub state: Account<'info, PointsState>,
    #[account(mut)]
    pub config: Account<'info, Config>,
    pub user1: Signer<'info>,
    pub user2: Signer<'info>,
}

#[account]
pub struct PointsState {
    pub count: u64,
    pub active: bool,
}

#[account]
pub struct PointsMetadata {
    pub owner: Pubkey,
    pub threshold: u64,
    pub active: bool,
}

#[account]
pub struct Config {
    pub param: u8,
    pub flag: bool,
}

#[error_code]
pub enum CustomError {
    DuplicateAccount,
}
